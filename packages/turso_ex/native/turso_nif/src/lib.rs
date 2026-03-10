use rustler::{Atom, Binary, Encoder, Env, ResourceArc, Term};
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

mod atoms {
    rustler::atoms! {
        ok,
        error,
        blob,
        columns,
        rows,
        num_rows,
    }
}

fn runtime() -> &'static Runtime {
    // Safe: on_load initializes RUNTIME before any NIF can be called.
    // If this is None, the NIF module failed to load and we can't be here.
    RUNTIME
        .get()
        .expect("RUNTIME not initialized (NIF load failed?)")
}

// Safety cap: conn_query buffers all rows in memory before encoding to BEAM terms.
// Without a limit, a large SELECT could exhaust memory and crash the node.
const MAX_ROWS: usize = 10_000;

// --- Resources ---

struct DbResource {
    db: turso::Database,
}

struct ConnResource {
    conn: turso::Connection,
}

// --- Owned value type for safe encoding after async work ---

enum OwnedValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl Encoder for OwnedValue {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            OwnedValue::Null => rustler::types::atom::nil().encode(env),
            OwnedValue::Integer(i) => i.encode(env),
            OwnedValue::Real(f) => f.encode(env),
            OwnedValue::Text(s) => s.encode(env),
            OwnedValue::Blob(b) => b.as_slice().encode(env),
        }
    }
}

fn to_owned(val: turso::Value) -> OwnedValue {
    match val {
        turso::Value::Null => OwnedValue::Null,
        turso::Value::Integer(i) => OwnedValue::Integer(i),
        turso::Value::Real(f) => OwnedValue::Real(f),
        turso::Value::Text(s) => OwnedValue::Text(s),
        turso::Value::Blob(b) => OwnedValue::Blob(b),
    }
}

// --- Parameter decoding ---

fn decode_term_to_value(term: Term) -> Result<turso::Value, rustler::Error> {
    // nil
    if rustler::types::atom::nil().eq(&term) {
        return Ok(turso::Value::Null);
    }

    // booleans (must check before integer since true/false are atoms)
    if let Ok(b) = term.decode::<bool>() {
        return Ok(turso::Value::Integer(if b { 1 } else { 0 }));
    }

    // integer
    if let Ok(i) = term.decode::<i64>() {
        return Ok(turso::Value::Integer(i));
    }

    // float
    if let Ok(f) = term.decode::<f64>() {
        return Ok(turso::Value::Real(f));
    }

    // {:blob, binary}
    if let Ok((tag, data)) = term.decode::<(Atom, Binary)>() {
        if tag == atoms::blob() {
            return Ok(turso::Value::Blob(data.as_slice().to_vec()));
        }
        return Err(rustler::Error::BadArg);
    }

    // string (UTF-8 binary)
    if let Ok(s) = term.decode::<String>() {
        return Ok(turso::Value::Text(s));
    }

    Err(rustler::Error::BadArg)
}

// TODO: support named parameters (map/keyword list -> Params::Named)
fn decode_params(terms: Vec<Term>) -> Result<turso::params::Params, rustler::Error> {
    if terms.is_empty() {
        return Ok(turso::params::Params::None);
    }
    let values: Vec<turso::Value> = terms
        .into_iter()
        .map(decode_term_to_value)
        .collect::<Result<_, _>>()?;
    Ok(turso::params::Params::Positional(values))
}

// --- NIFs ---

#[rustler::nif(schedule = "DirtyIo")]
fn db_open<'a>(env: Env<'a>, path: String) -> Term<'a> {
    let result = runtime().block_on(turso::Builder::new_local(&path).build());

    match result {
        Ok(db) => (atoms::ok(), ResourceArc::new(DbResource { db })).encode(env),
        Err(e) => (atoms::error(), format!("db_open: {e}")).encode(env),
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn db_connect<'a>(env: Env<'a>, db: ResourceArc<DbResource>) -> Term<'a> {
    match db.db.connect() {
        Ok(conn) => (atoms::ok(), ResourceArc::new(ConnResource { conn })).encode(env),
        Err(e) => (atoms::error(), format!("db_connect: {e}")).encode(env),
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn conn_execute<'a>(
    env: Env<'a>,
    conn: ResourceArc<ConnResource>,
    sql: String,
    params: Vec<Term<'a>>,
) -> Term<'a> {
    let turso_params = match decode_params(params) {
        Ok(p) => p,
        Err(_) => {
            return (atoms::error(), "conn_execute: invalid parameter type").encode(env);
        }
    };

    let result = runtime().block_on(conn.conn.execute(&sql, turso_params));

    match result {
        Ok(count) => (atoms::ok(), count).encode(env),
        Err(e) => (atoms::error(), format!("conn_execute: {e}")).encode(env),
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn conn_query<'a>(
    env: Env<'a>,
    conn: ResourceArc<ConnResource>,
    sql: String,
    params: Vec<Term<'a>>,
) -> Term<'a> {
    let turso_params = match decode_params(params) {
        Ok(p) => p,
        Err(_) => {
            return (atoms::error(), "conn_query: invalid parameter type").encode(env);
        }
    };

    // All async work in one block_on call. Collect Rust-owned data first,
    // then encode to Elixir terms after the async work completes.
    let result = runtime().block_on(async {
        let mut rows = conn.conn.query(&sql, turso_params).await?;
        let columns = rows.column_names();
        let col_count = rows.column_count();
        // TODO: replace row cap with streaming to avoid buffering large result sets
        let mut collected: Vec<Vec<OwnedValue>> = Vec::new();

        while let Some(row) = rows.next().await? {
            if collected.len() >= MAX_ROWS {
                return Err(turso::Error::Error(format!(
                    "conn_query: result set exceeded {MAX_ROWS} row cap"
                )));
            }
            let mut vals = Vec::with_capacity(col_count);
            for i in 0..col_count {
                vals.push(to_owned(row.get_value(i)?));
            }
            collected.push(vals);
        }

        Ok::<_, turso::Error>((columns, collected))
    });

    match result {
        Ok((columns, rows)) => {
            let num_rows = rows.len();
            let map = match rustler::types::map::map_new(env)
                .map_put(atoms::columns().encode(env), columns.encode(env))
                .and_then(|m| m.map_put(atoms::rows().encode(env), rows.encode(env)))
                .and_then(|m| m.map_put(atoms::num_rows().encode(env), num_rows.encode(env)))
            {
                Ok(m) => m,
                Err(_) => {
                    return (atoms::error(), "conn_query: failed to build result map").encode(env)
                }
            };
            (atoms::ok(), map).encode(env)
        }
        Err(e) => (atoms::error(), format!("conn_query: {e}")).encode(env),
    }
}

#[allow(non_local_definitions)]
fn on_load(env: Env, _info: Term) -> bool {
    let _ = rustler::resource!(DbResource, env);
    let _ = rustler::resource!(ConnResource, env);

    match Runtime::new() {
        Ok(rt) => {
            let _ = RUNTIME.set(rt);
            true
        }
        Err(_) => false,
    }
}

rustler::init!("Elixir.TursoEx.Native", load = on_load);
