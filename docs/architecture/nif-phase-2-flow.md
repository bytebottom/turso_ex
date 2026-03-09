# NIF Phase 2 Flow

Phase 2 adds the first real local database path for `turso_ex`.

## Data Flow

```text
Elixir
  -> TursoEx facade / low-level NIF wrapper
    -> Rustler NIF entrypoint
      -> decode Elixir params into turso::Value
      -> Tokio runtime block_on(...)
        -> Turso database / connection call
        -> optional row collection into Rust-owned values
      -> encode {:ok, result} | {:error, reason}
```

## Resource Model

```text
db_open/1
  -> ResourceArc<DbResource>

db_connect/1
  -> ResourceArc<ConnResource>
```

`DbResource` wraps `turso::Database`.

`ConnResource` wraps `turso::Connection`.

The public `%TursoEx.Conn{}` keeps both handles alive internally:

```text
open/1
  -> db_open/1
  -> db_connect/1
  -> %TursoEx.Conn{db: ..., conn: ...}
```

No global registry, no extra connection mutex layer.

## Query Path

```text
conn_query/3
  -> decode params
  -> conn.query(sql, params)
  -> rows.column_names()
  -> rows.next().await loop
  -> fully consume iterator inside block_on
  -> OwnedValue / QueryResult
  -> encode {:ok, %{columns: [...], rows: [[...]], num_rows: n}}
```

## Execute Path

```text
conn_execute/3
  -> decode params
  -> conn.execute(sql, params)
  -> encode {:ok, rows_changed}
```

## Failure Paths

```text
bad Elixir param
  -> {:error, "param_decode: ..."}

db or query failure from Turso
  -> {:error, "db_open|db_connect|conn_execute|conn_query: ..."}

row/value conversion failure
  -> {:error, "row_collect: column ..."}
```

## Boundaries

- Do not create Elixir `Term`s inside async row iteration.
- Do not keep env-bound data alive across `.await`.
- Do not return a lazy or streaming rows handle to Elixir in Phase 2.
- Boolean input is allowed as `0/1` convenience only. Reads come back as integers.
