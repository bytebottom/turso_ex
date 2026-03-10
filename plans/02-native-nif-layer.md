# Plan 02: Native NIF Layer

This plan covers the Rust-side Phase 2 implementation.

## Goal

Implement the local Turso path in Rust with correct resources, parameter decoding, row collection, and tagged-tuple returns.

## Design Rules

- use `ResourceArc` for database and connection handles
- no global registries
- no extra mutex wrapper around `turso::Connection`
- collect Rust-owned values first, then encode at the NIF boundary
- never build Elixir `Term`s across `.await`

## Error Boundary

Rust and Elixir have different jobs here.

- Rust returns tagged tuples with operation-scoped messages, for example `{:error, "db_open: ..."}`.
- Elixir normalizes those into `%TursoEx.Error{}` later.

Do not blur those layers.

## Resource Model

- `DbResource { db: turso::Database }`
- `ConnResource { conn: turso::Connection }`

## Data Boundary

Internal owned values:

- `Null`
- `Integer(i64)`
- `Real(f64)`
- `Text(String)`
- `Blob(Vec<u8>)`

Result map:

- `columns`
- `rows`
- `num_rows`

NIF query return shape:

- `{:ok, %{columns: [...], rows: [[...]], num_rows: n}}`
- `{:error, "conn_query: ..."}`

## Decode Note

`decode_term_to_value/1` uses this decode order:

- `nil`
- `bool -> 0/1` (must precede integer, since true/false are atoms that Rustler can decode as i64)
- `i64`
- `f64`
- `{:blob, binary}` (must precede String, so tagged blobs are not swallowed as text)
- `String` (UTF-8 binary)

Untagged non-UTF-8 binaries are rejected with `BadArg`. This is intentional:
Elixir strings and raw bytes are both binaries at the BEAM level, so there is
no way to distinguish text from blob without an explicit tag. Silently guessing
based on UTF-8 validity would store data under the wrong type.

## Action Items

### Resources and registration

- [ ] 1. Remove placeholder `db_push` and `db_pull` stubs for Phase 2.
- [ ] 2. Define `DbResource` and `ConnResource`.
- [ ] 3. Register both resources in `on_load`.
- [ ] 4. Keep `rustler::init!("Elixir.TursoEx.Native", load = on_load)` aligned with the Elixir module name.

### Shared helpers

- [ ] 5. Add helpers for consistent tagged tuple returns.
- [ ] 6. Add operation-scoped error messages on the Rust side.
- [ ] 7. Implement `decode_term_to_value/1`.
- [ ] 8. Implement owned result types for row collection.
- [ ] 9. Implement row collection from `turso::Rows`, including column names and owned scalar conversion.
  Consume the full async iterator inside `RUNTIME.block_on(...)`. Do not return a lazy or streaming rows handle to Elixir in Phase 2.

### NIFs

- [ ] 10. Implement `db_open/1` using `Builder::new_local(...).build().await`.
- [ ] 11. Implement `db_connect/1` using `db.connect()`.
- [ ] 12. Implement `conn_execute/3` using `conn.execute(&sql, decoded_params).await`.
- [ ] 13. Implement `conn_query/3` using `conn.query(&sql, decoded_params).await`.

## Exit Criteria

- the Rust side opens, connects, executes, and queries locally
- errors come back as tagged tuples
- row collection does not depend on Elixir env-bound state
