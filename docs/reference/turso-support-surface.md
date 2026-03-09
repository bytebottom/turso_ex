# Turso Support Surface

This document separates three things that are easy to conflate:

- Turso engine compatibility with SQLite
- the public Rust SDK surface from the `turso` crate
- the Elixir wrapper surface exposed by `turso_ex`

If these are not documented separately, support claims get sloppy fast.

## Public API Design Rule

`turso_ex` should be documented and judged primarily as an Elixir library, not as a Rust NIF wrapper.

That means:

- the canonical docs should show `TursoEx.open/query/execute/one`
- `transaction/2` is an end-state API target, not a Phase 2 feature
- `TursoEx.Native` is an implementation detail
- `TursoEx.Driver` may exist later for parity and testing, but should not dominate the happy path

The public API should revolve around three stable types:

- `%TursoEx.Conn{}`
- `%TursoEx.Result{}`
- `%TursoEx.Error{}`

## Repository And Package Rule

Keep this as a mono-repo with separate packages, not an umbrella.

Target shape:

```text
repo/
  Cargo.toml
  packages/
    turso_ex/
      mix.exs
      lib/
      test/
      native/turso_nif/
    ecto_turso_ex/
      mix.exs
  docs/architecture/
  docs/reference/
```

Rules:

- the repo root is a workspace, not a Mix app
- the core package is `packages/turso_ex`
- the Ecto adapter package is `packages/ecto_turso_ex`
- the Rust NIF crate lives in `packages/turso_ex/native/turso_nif`
- the core package should not depend on `Ecto`, `EctoSQL`, or `DBConnection`
- do not convert the repo into an umbrella unless deployment needs, not library ergonomics, somehow force it later

## Sources

- Upstream compatibility matrix: https://github.com/tursodatabase/turso/blob/main/COMPAT.md
- Local crate version in this repo: `turso = 0.5.1-pre.1`

## Rule Of Thumb

### Engine compatibility

If a feature is expressed as SQL and the engine supports it, we usually do **not** need a dedicated NIF.

Examples:

- `RETURNING`
- many `PRAGMA` statements
- JSON functions
- UUID functions
- vector functions
- percentile functions
- triggers
- `ATTACH`
- Turso FTS syntax

These should mostly ride through `conn_execute/3` and `conn_query/3`.

### SDK parity

If a feature is a distinct Rust method or type, we do need deliberate wrapper support.

Examples:

- `execute_batch`
- `prepare`
- `prepare_cached`
- `query_row`
- `pragma_query`
- `pragma_update`
- `last_insert_rowid`
- `cacheflush`
- `is_autocommit`
- `busy_timeout`
- transactions
- sync database builder and sync operations

## Public Data Contracts

### `%TursoEx.Conn{}`

- opaque connection capability
- the main thing users pass around
- should remain stable even if local and remote modes are added later

### `%TursoEx.Result{}`

- positional row container
- `columns`, `rows`, `num_rows`
- rows are lists, not maps, in the base API

### `%TursoEx.Error{}`

- stable error struct, not ad hoc tuples all the way up
- small stable `kind` taxonomy is preferred over free-form error classification

Recommended initial `kind` values:

- `:invalid_argument`
- `:sql_error`
- `:busy`
- `:misuse`
- `:internal`

## Public API Constraints

To keep Phase 2 clean:

- `open/1` returns a ready-to-use connection capability
- positional params are the public default
- named params are not part of the initial public contract
- result rows are positional
- text and blob values both appear as Elixir binaries

## Upstream Constraints To Document Honestly

From upstream compatibility docs and local crate behavior:

- no concurrent access from multiple processes
- `VACUUM` unsupported
- rollback-journal modes are not the target, WAL is the intended mode
- SQLite FTS3/FTS4/FTS5 unsupported, use Turso FTS instead
- dynamic loadable SQLite extensions are unsupported
- savepoint support is not safe to promise yet

## Ambiguous Or Risky Areas

### Savepoints

The upstream compatibility doc has inconsistent status text around `SAVEPOINT` and `RELEASE SAVEPOINT`.

Separately, the local `turso` crate transaction tests include this note:

- Turso does not currently support savepoints in the helper transaction path

Practical rule:

- do not claim savepoint support in `turso_ex` docs until we verify it with targeted tests

### Extensions

The upstream doc distinguishes between:

- built-in Turso extensions and functions, which are available through SQL
- dynamic SQLite loadable extensions, which are not supported

Practical rule:

- document built-in Turso SQL extensions as SQL features
- do not imply `.so` / `.dll` extension loading support

### FTS

Upstream Turso FTS is not SQLite FTS5 compatibility. It is a Turso-specific feature set.

Practical rule:

- document FTS as supported through Turso syntax
- do not advertise SQLite FTS5 parity

## Public `turso` Crate API Inventory

This is the main SDK surface we should eventually cover.

### Local builder and database

- `Builder::new_local`
- experimental builder toggles for encryption, triggers, attach, strict, custom types, index method, materialized views
- `Builder::with_io`
- `Builder::build`
- `Database::connect`

### Connection

- `query`
- `execute`
- `execute_batch`
- `prepare`
- `prepare_cached`
- `pragma_query`
- `pragma_update`
- `last_insert_rowid`
- `cacheflush`
- `is_autocommit`
- `busy_timeout`

### Statement and rows

- statement execution and querying
- `query_row`
- column names and metadata

### Transactions

- `transaction`
- `transaction_with_behavior`
- `unchecked_transaction`
- `commit`
- `rollback`
- `finish`
- drop behavior and transaction behavior controls

### Sync / remote

- `sync::Builder::new_remote`
- remote URL, auth token, client name, long poll, bootstrap, partial sync, remote encryption
- `build`
- `push`
- `pull`
- `checkpoint`
- `stats`
- `connect`

## Recommended Support Phases

### Phase 2

Public facade plus local open / execute / query, all in `packages/turso_ex`.

### Phase 3

Local utility and statement parity, and introduce `TursoEx.Driver` only if that work actually needs a middle layer. `packages/ecto_turso_ex/` can remain scaffolded, but the adapter should still wait until the core package shape is stable enough to build on.

### Phase 4

Transactions.

### Phase 5

Remote and sync.

### Phase 6

Compatibility probes and support docs maintenance.

## Documentation Policy

When documenting support:

- say "supported through SQL" for engine-level features
- say "wrapped by `turso_ex`" for explicit Elixir API coverage
- say "unsupported upstream" when the engine does not support it
- say "not yet exposed by `turso_ex`" when Turso supports it but our wrapper does not yet
- say "ambiguous, requires verification" when upstream docs and local behavior do not line up cleanly

When documenting examples:

- lead with the public `TursoEx` facade
- mention `TursoEx.Driver` only when discussing parity work or advanced control
- mention `TursoEx.Native` only when discussing internals, debugging, or implementation details

Practical public API rule:

- `TursoEx.open/1` should return a ready-to-use connection capability
- avoid separate public `db` and `conn` choreography unless the API proves it truly needs both

## Non-goals

At least for the early phases, this library is not:

- an ORM
- a DBConnection adapter
- a schema-aware type-casting layer
- a promise to emulate every SQLite surface beyond what upstream Turso actually supports

## Versioning Note

Support claims should be tied to the checked-in `turso` crate version in this repo, not to a vague idea of "Turso in general".
