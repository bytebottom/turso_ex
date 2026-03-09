use rustler::{Env, NifResult, Term};
use std::sync::LazyLock;
use tokio::runtime::Runtime;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().expect("did not get runtime"));

#[rustler::nif(schedule = "DirtyIo")]
fn db_open(_path: String, _sync_url: String, _auth_token: String) -> NifResult<String> {
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
) -> NifResult<(Vec<String>, Vec<Vec<Term>>)> {
    Ok((vec![], vec![]))
}

#[rustler::nif(schedule = "DirtyIo")]
fn db_push(_db: String) -> NifResult<()> {
    Ok(())
}

#[rustler::nif(schedule = "DirtyIo")]
fn db_pull(_db: String) -> NifResult<()> {
    Ok(())
}

fn on_load(_env: Env, _info: Term) -> bool {
    true
}

rustler::init!("Elixir.TursoEx.Native", load = on_load);
