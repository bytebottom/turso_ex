# Plan 01: Contract

This is not an execution plan. It is the contract for the work that follows.

Nothing else should move until this stays stable.

## Goal

Build `turso_ex` as an Elixir-first library with a small public API and full Turso support underneath over time.

The NIF layer matters, but it is not the product.

## Package Boundaries

- the core package lives in `packages/turso_ex`
- the adapter package lives in `packages/ecto_turso_ex`
- the Rust NIF crate lives in `packages/turso_ex/native/turso_nif`
- the repo root is a workspace, not a Mix app
- do not convert the repo into an umbrella

## Public API Bar

This is the target public surface:

```elixir
{:ok, conn} = TursoEx.open(":memory:")
{:ok, conn} = TursoEx.open(path: ":memory:")
{:ok, result} = TursoEx.query(conn, "select 1", [])
{:ok, count} = TursoEx.execute(conn, "insert into items(name) values (?)", ["x"])
{:ok, value} = TursoEx.one(conn, "select count(*) from items", [])
```

Phase 4 target:

```elixir
TursoEx.transaction(conn, fn tx ->
  :ok = TursoEx.execute!(tx, "insert into items(name) values (?)", ["y"])
  TursoEx.one!(tx, "select count(*) from items", [])
end)
```

Public types:

- `%TursoEx.Conn{}`
- `%TursoEx.Result{}`
- `%TursoEx.Error{}`

## Scope

### In for Phase 2

- local open / connect / execute / query
- public `open/query/execute/one`
- package-boundary cleanup so the core package stays free of Ecto concerns
- local-only tests
- architecture/data-flow documentation
- support-surface documentation

### Out for Phase 2

- sync and remote replicas
- transactions
- prepared statements
- DBConnection and Ecto adapter implementation
- type loaders / dumpers
- telemetry and dashboards

## Contracts

### Internal native contract

All public NIFs return tagged tuples:

- `{:ok, value}`
- `{:error, reason}`

Phase 2 native API:

- `db_open/1`
- `db_connect/1`
- `conn_execute/3`
- `conn_query/3`

### Public Elixir contract

- `TursoEx.open/1`
- `TursoEx.query/3`
- `TursoEx.execute/3`
- `TursoEx.one/3`

Rules:

- `open/1` accepts either a bare path string or `path: ...`
- `open/1` returns a ready-to-use connection
- `conn_execute/3` does not auto-route SQL
- `conn_query/3` is the row-returning path
- booleans may encode as `0/1`, but raw reads come back as integers
- `conn_query/3` returns `{:ok, %{columns: [...], rows: [[...]], num_rows: n}}`
- `one/3` returns `{:ok, value}` only for exactly one row and one column
- `one/3` returns `{:ok, nil}` for zero rows
- `one/3` returns `{:error, %TursoEx.Error{}}` for multiple rows or multiple columns

## Public Type Rules

`%TursoEx.Conn{}`:

- opaque
- the capability users pass around
- stable outer type for future local and remote modes
- retains both the database handle and the connection handle internally
- keeps the database handle alive so later sync and remote work does not force a public shape change

`%TursoEx.Result{}`:

- `columns`
- `rows`
- `num_rows`
- positional rows, not maps

`%TursoEx.Error{}`:

- `operation`
- `message`
- `kind`
- `details`

Initial `kind` set:

- `:invalid_argument`
- `:constraint`
- `:sql_error`
- `:busy`
- `:misuse`
- `:internal`

## Exit Criteria

- package boundaries are stable
- the public contract is clear
- the team can start the NIF work without arguing about API shape every hour
