use rustler::{Env, NifResult, Term};
use std::sync::LazyLock;
use tokio::runtime::Runtime;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().expect("did not get runtime"));

#[derive(rustler::NifMap)]
struct QueryResult {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    num_rows: usize,
}

#[rustler::nif(schedule = "DirtyIo")]
fn db_open(_path: String) -> NifResult<String> {
    Ok("placeholder".to_string())
}

#[rustler::nif(schedule = "DirtyIo")]
fn db_connect(_db: String) -> NifResult<String> {
    Ok("placeholder".to_string())
}

#[rustler::nif(schedule = "DirtyIo")]
fn conn_execute(_conn: String, _sql: String, _params: Vec<Term>) -> NifResult<u64> {
    Ok(0)
}

#[rustler::nif(schedule = "DirtyIo")]
fn conn_query(
    _conn: String,
    _sql: String,
    _params: Vec<Term>,
) -> NifResult<QueryResult> {
    Ok(QueryResult {
        columns: vec![],
        rows: vec![],
        num_rows: 0,
    })
}

fn on_load(_env: Env, _info: Term) -> bool {
    true
}

rustler::init!("Elixir.TursoEx.Native", load = on_load);
