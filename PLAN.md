# Plan: Public-first TursoEx API with full Turso coverage underneath

Implement `turso_ex` as an Elixir-first library with a small, obvious public API and full Turso support underneath.

The NIF layer matters, but it is not the product. The product is the Elixir interface people will reach for at 2am and understand without reading the source.

The goal is threefold:

- define a clean public API that feels native in Elixir
- build the NIF and driver layers underneath it without leaking them everywhere
- keep the roadmap aligned with the real Turso engine and Rust SDK support surface
- keep the repo mono-repo friendly without turning it into an umbrella

## Recommendation

Use a mono-repo with separate packages, not an umbrella:

- the core package lives in `packages/turso_ex`
- `ecto_turso_ex` lives in `packages/ecto_turso_ex`
- the Rust NIF crate lives in `packages/turso_ex/native/turso_nif`

Inside the core package, use a simple layer design:

- `TursoEx.Native`, private or low-visibility, thin NIF boundary
- `TursoEx.Driver`, optional SDK-shaped Elixir wrapper for parity and testing
- `TursoEx`, the canonical public API

Internally, use `ResourceArc` resources for database and connection handles and collect query results into plain Rust-owned data before encoding them back to Elixir.

Use `ecto_libsql` as a reference for:

- parameter decode order
- result map shape (`columns`, `rows`, `num_rows`)
- pragmatic error messages

Do **not** copy these parts from `ecto_libsql` in this phase:

- string-ID registries
- extra mutex layers around connections
- `should_use_query()` routing inside `conn_execute/3`

Use the upstream Turso compatibility matrix as a documentation source, not as a 1:1 checklist of NIFs. Most SQL compatibility comes from the engine once SQL is passed through correctly. The wrapper still needs explicit work for Rust SDK surface area such as transactions, sync, prepared statements, and connection utilities.

Do not use an umbrella for this project. The core library and the future Ecto adapter need clean release and dependency boundaries. Being in one repo is enough. The umbrella buys very little and makes it easier to blur those boundaries.

## Public API principles

This is the bar:

- one obvious happy path
- SQL-first ergonomics
- stable result and error types
- advanced control without infecting the beginner path
- low ceremony
- capability honesty in docs

If the public API feels like a renamed FFI, it failed.

## Target Public API

The public surface should look roughly like this once the roadmap is complete:

```elixir
{:ok, conn} = TursoEx.open(":memory:")

{:ok, %TursoEx.Result{} = result} =
  TursoEx.query(conn, "select 1 as x", [])

{:ok, count} =
  TursoEx.execute(conn, "insert into users(name) values (?)", ["petar"])

{:ok, value} =
  TursoEx.one(conn, "select count(*) from users", [])

# Phase 4 target:
# TursoEx.transaction(conn, fn tx ->
#   :ok = TursoEx.execute!(tx, "insert into users(name) values (?)", ["jose"])
#   TursoEx.one!(tx, "select count(*) from users", [])
# end)
```

Public modules we should aim for:

- `TursoEx`
- `TursoEx.Result`
- `TursoEx.Error`
- `TursoEx.Statement` later, if prepared statements become public

Internal modules:

- `TursoEx.Native`
- `TursoEx.Driver` later, if Phase 3 parity work actually needs it

## Repository Shape

Target repository layout:

```text
repo/
  packages/
    turso_ex/
      mix.exs
      lib/
      test/
      native/turso_nif/
    ecto_turso_ex/
      mix.exs
  docs/
```

Why this shape:

- one repo keeps development fast
- separate Mix projects keep package boundaries honest
- `ecto_turso_ex` can depend on `turso_ex` like any other consumer
- the core package can stay free of `Ecto`, `DBConnection`, and adapter-driven API compromise
- the adapter package can evolve on its own without pretending to be Phase 2 core functionality

## What "full Turso support" means

We need to document and implement three different surfaces, because they are not the same thing:

### 1. Engine compatibility

This is what `COMPAT.md` describes: SQL statements, PRAGMAs, functions, extensions, and SQLite compatibility behavior inside the Turso engine.

Most of this does **not** require a bespoke NIF. Once `conn_execute/3` and `conn_query/3` work correctly, callers can already use a large part of the documented SQL surface through plain SQL strings.

Examples:

- `ATTACH`, triggers, JSON functions, UUID functions, vector functions, percentile functions
- Turso FTS syntax
- CSV virtual tables
- many supported PRAGMAs

### 2. Rust SDK surface

This is the public API of the `turso` crate. This **does** require explicit NIF coverage if we want `turso_ex` to expose the full Turso API cleanly.

Main groups:

- local builder and experimental feature toggles
- connection operations
- statement operations
- transaction operations
- sync / remote database operations
- connection metadata and utility methods

### 3. `turso_ex` public wrapper surface

This is our Elixir API. It should be intentionally designed, not a random pile of thin wrappers.

The correct approach is:

- keep the NIF layer close to the Rust SDK
- expose SQL engine features mostly through `query/execute/one/transaction`
- add Elixir-friendly wrappers where the Rust API shape would otherwise leak awkwardly

In Phase 2, the implemented runtime surface stays in `packages/turso_ex` with `TursoEx.Native` plus `TursoEx`.
Only introduce `TursoEx.Driver` in Phase 3 if the SDK parity work shows a real need for it.
Keep `ecto_turso_ex` scaffolded as a sibling package, but do not let it drive the core API before the core package is stable enough to support it cleanly.

The public API should be shaped around user intent:

- `open`
- `query`
- `execute`
- `one`
- `all` later if it earns its keep
- `transaction`
- `prepare` later

## Core Public Types

If these three are clean, the library will feel clean:

- `%TursoEx.Conn{}`
- `%TursoEx.Result{}`
- `%TursoEx.Error{}`

Phase 2 should treat these as the primary API design objects, not as an afterthought on top of the NIFs.

## Upstream constraints we must document honestly

These are upstream engine limitations or caveats we should not hide:

- no concurrent access from multiple processes
- `VACUUM` unsupported
- SQLite FTS3/FTS4/FTS5 unsupported, Turso FTS is the supported path
- dynamic SQLite loadable extensions (`.so` / `.dll`) unsupported
- savepoint support is ambiguous in `COMPAT.md`, and the Rust transaction tests explicitly note that Turso does not currently support savepoints in the transaction helper path

Reference: [docs/reference/turso-support-surface.md](docs/reference/turso-support-surface.md)

## Scope

**In for Phase 2:** NIF primitives for local open/connect/execute/query, the first public `TursoEx` facade centered on `open/query/execute/one`, package-boundary cleanup so `packages/turso_ex` is not coupled to Ecto, local-only tests, one architecture/data-flow diagram, operation-scoped error messages, support-surface documentation.

**Out for Phase 2 implementation:** sync (`db_push`/`db_pull`), remote replicas, transactions, prepared statements, cursors, batch execution, DBConnection, Ecto adapter, type loaders/dumpers, telemetry, dashboards, umbrella conversion.

**In for the overall roadmap:** full Rust SDK parity, documented engine compatibility notes, and compatibility probes for important Turso features.

## Phase 2 Public Contract

Phase 2 should define two contracts:

### 1. Internal NIF contract

All public NIFs return tagged tuples:

- `{:ok, value}`
- `{:error, reason}`

Expected runtime failures must not raise via `rustler::Error::Term`. Reserve actual NIF exceptions for decoding bugs or impossible internal failures.

API for this phase:

- `db_open(path, sync_url, auth_token) :: {:ok, db_ref} | {:error, binary}`
- `db_connect(db_ref) :: {:ok, conn_ref} | {:error, binary}`
- `conn_execute(conn_ref, sql, params) :: {:ok, non_neg_integer} | {:error, binary}`
- `conn_query(conn_ref, sql, params) :: {:ok, %{columns: [binary], rows: [[term]], num_rows: non_neg_integer}} | {:error, binary}`

These functions belong under `TursoEx.Native` and should not be the main API users see in docs.

### 2. Public Elixir contract

Public functions for Phase 2:

- `TursoEx.open(path_or_opts) :: {:ok, conn} | {:error, TursoEx.Error.t()}`
- `TursoEx.query(conn, sql, params \\ []) :: {:ok, TursoEx.Result.t()} | {:error, TursoEx.Error.t()}`
- `TursoEx.execute(conn, sql, params \\ []) :: {:ok, non_neg_integer} | {:error, TursoEx.Error.t()}`
- `TursoEx.one(conn, sql, params \\ []) :: {:ok, term | nil} | {:error, TursoEx.Error.t()}`
- bang variants later if they are trivial to add cleanly

Public structs for Phase 2:

- `%TursoEx.Conn{}`
- `%TursoEx.Result{columns, rows, num_rows}`
- `%TursoEx.Error{operation, message, kind, details}`

`one/3` is worth introducing early. It makes the library feel much better than forcing callers to manually unpack a result struct for the most common scalar read path.

`open/1` should return a ready-to-use connection. Separate `db` and `conn` concepts belong in the native layer unless the public API later proves it really needs both.

Phase 2 should also set these expectations clearly:

- `%TursoEx.Conn{}` is the one public capability users pass around
- `%TursoEx.Result{}` is the stable row-returning shape
- `%TursoEx.Error{}` is the stable failure shape

Contract notes:

- `sync_url` and `auth_token` stay in the `db_open/3` signature for forward compatibility, but are ignored in this local-only phase.
- `conn_execute/3` is the write and DDL path. It does not auto-route `SELECT`, `PRAGMA`, or `RETURNING` statements.
- `conn_query/3` is the row-returning path. Callers should use it for `SELECT`, row-returning `PRAGMA`, and `... RETURNING ...`.
- Boolean input may be accepted as convenience and encoded as `0` or `1`, but reads come back as integers. There is no boolean round-trip at the raw NIF layer.

## Data Model At The Boundary

### Resources

- `DbResource { db: turso::Database }`
- `ConnResource { conn: turso::Connection }`

No global registries. No extra `Arc<Mutex<_>>` around `turso::Connection`. The Turso crate already owns the concurrency story and `Connection::query/execute` take `&self`.

### Query Result Encoding

Collect async query output into plain Rust-owned values first, then encode once at the NIF boundary.

Suggested internal types:

- `OwnedValue` enum: `Null | Integer(i64) | Real(f64) | Text(String) | Blob(Vec<u8>)`
- `QueryResult` map struct: `columns`, `rows`, `num_rows`

Use Rustler derive support where it helps:

- `#[derive(NifMap)]` for the result map
- `#[derive(NifUntaggedEnum)]` for owned scalar values, or a small manual `Encoder` if that is cleaner

Important boundary rule:

- do not build `Term`s or `Binary`s while iterating async rows
- do not hold Elixir env-bound data across `.await`

### Public result and error types

Phase 2 should introduce stable Elixir-facing types immediately:

- `%TursoEx.Conn{}`
- `%TursoEx.Result{columns, rows, num_rows}`
- `%TursoEx.Error{operation, message, kind, details}`

Even if `details` starts small, the struct shape should be stable now.

`%TursoEx.Conn{}` should be an opaque wrapper around the native resource, not a bag of public fields.

`%TursoEx.Conn{}` design rules:

- opaque struct, not user-constructed
- stable outer type even if the underlying native resources evolve
- capable of representing future local and remote modes without API breakage
- safe to pass between BEAM processes as a capability, but docs must not overclaim underlying engine semantics

`%TursoEx.Result{}` design rules:

- `columns :: [String.t()]`
- `rows :: [[term()]]`
- `num_rows :: non_neg_integer()`
- keep row shape boring and positional in Phase 2
- do not add map-shaped rows in Phase 2
- if `all/3` is added later, it should be a convenience over this stable shape, not a competing primitive

`%TursoEx.Error{}` design rules:

- `operation` should be machine-meaningful and stable
- `message` should be human-readable
- `kind` should be a small stable taxonomy
- `details` can grow over time without breaking callers

Recommended initial `kind` set:

- `:invalid_argument`
- `:sql_error`
- `:busy`
- `:misuse`
- `:internal`

That buys us:

- better docs
- better pattern matching
- future room for richer errors without breaking the API

### Parameters and rows

Phase 2 should make a few intentionally narrow choices:

- positional parameters are the public default
- named parameters are not part of the Phase 2 public contract
- row results are positional lists, not maps
- text and blob both arrive in Elixir as binaries

This should be documented as a deliberate simplification, not an accidental omission.

If named parameters are added later, they should arrive as an explicit extension to `query/execute`, not as undocumented partial support.

## Architecture

Reference diagram: [docs/architecture/nif-phase-2-flow.md](docs/architecture/nif-phase-2-flow.md)

ASCII overview:

```text
repo root (workspace)
  -> packages/turso_ex
    -> TursoEx
      -> TursoEx.Native
        -> packages/turso_ex/native/turso_nif
  -> packages/ecto_turso_ex later, as a sibling Mix project

Elixir caller
  -> TursoEx.open/1
    -> facade normalization
      -> Native db_open/3
      -> Native db_connect/1
      -> %TursoEx.Conn{}

Elixir caller
  -> TursoEx.execute/3 or TursoEx.query/3
    -> facade unwraps %TursoEx.Conn{}
      -> Native conn_execute/3 or conn_query/3
        -> Rust NIF decodes params
          -> tokio runtime block_on
            -> turso::Connection execute/query
            -> collect Rust-owned rows if query path
          -> encode tagged tuple back to Elixir
      -> normalize into TursoEx.Result / TursoEx.Error
```

Failure path:

```text
decode failure | Turso API error | row/value conversion error
  -> operation-scoped error string
  -> {:error, reason}
```

## Observability

This is a small change, so keep observability lightweight but deliberate.

- Prefix every returned error with the operation name, for example `db_open: ...`, `conn_execute: ...`, `conn_query: ...`, `param_decode: ...`, `row_collect: ...`
- Include value context where cheap and safe: parameter index, column index, column name when available
- Do not log SQL params or blobs in full

Also:

- add `Inspect` implementations for `%TursoEx.Conn{}`, `%TursoEx.Result{}`, and `%TursoEx.Error{}`
- `Inspect` for `%TursoEx.Conn{}` should be minimal and never leak internals
- `Inspect` for `%TursoEx.Error{}` should make debugging pleasant in `iex`

## Full API Coverage Roadmap

Phase 2 is only the foundation. To support the full Turso Rust API, we should stage the work like this:

### Phase 2: public facade plus local foundation

Expose:

- `TursoEx.open/1`
- `TursoEx.query/3`
- `TursoEx.execute/3`
- `TursoEx.one/3`

Implement underneath:

- `db_open/3`
- `db_connect/1`
- `conn_execute/3`
- `conn_query/3`

Goal:

- correct local DB open / connect / execute / query behavior
- correct parameter and row conversion
- a public API people would actually want to use
- accurate docs about what SQL compatibility is already available through plain SQL
- a core package that is not pretending to be an Ecto adapter already

### Phase 3: connection utility and statement parity

If the parity work justifies it, introduce `TursoEx.Driver` here as the SDK-shaped middle layer.

Expose the rest of the core local connection surface, mostly through `TursoEx.Driver` first:

- `conn_execute_batch/2`
- `conn_prepare/2`
- `conn_prepare_cached/2`
- `stmt_execute/2`
- `stmt_query/2`
- `stmt_query_row/2`
- `rows` / `statement` metadata as needed
- `conn_pragma_query/2`
- `conn_pragma_update/3`
- `conn_last_insert_rowid/1`
- `conn_cacheflush/1`
- `conn_is_autocommit/1`
- `conn_busy_timeout/2`

Reason:

- this is the actual SDK parity line for local connection usage
- many upstream `COMPAT.md` features are easiest to validate through these helpers
- these may later graduate into public `TursoEx` APIs only where the ergonomics justify it

Repository note:

- `packages/ecto_turso_ex/` can remain a thin scaffold in this phase
- real adapter implementation should wait until the core package shape has earned it

### Phase 4: transaction parity

Expose transaction support with clear semantics:

- public:
  - `TursoEx.transaction/2`
  - bang and non-bang transactional helpers as needed
- driver / parity layer:
  - `conn_transaction_begin/1`
  - `conn_transaction_begin_with_behavior/2`
  - `tx_commit/1`
  - `tx_rollback/1`
  - `tx_finish/1`
  - `tx_set_drop_behavior/2`
  - transaction-scoped execute/query helpers, or transaction resources that reuse connection helpers

Documentation requirement:

- call out the current savepoint limitation explicitly
- do not overclaim nested transaction support
- keep transaction resources out of the main public API unless forced by a real use case

### Phase 5: sync and remote parity

Expose the `turso::sync` surface:

- remote builder path
- `db_push/1`
- `db_pull/1`
- `db_checkpoint/1`
- `db_stats/1`
- remote connect flow
- auth token, remote URL, long poll, client name, bootstrap flags, remote encryption

Public API target:

- `TursoEx.open/1` should eventually accept local and remote modes cleanly without turning into a keyword-list junk drawer
- if needed, split configuration structs from convenience options
- `%TursoEx.Conn{}` should survive this expansion without changing its role in the public API

### Phase 6: compatibility probes and support docs

Add a documented compatibility probe suite based on upstream `COMPAT.md` categories:

- statements and PRAGMAs we care about
- important scalar / JSON / UUID / vector / time / percentile functions
- Turso FTS behavior
- attachment / triggers / strict tables / extension notes
- explicit unsupported features we want to document instead of hand-wave

The probe suite should not try to reproduce the entire upstream matrix. It should cover:

- features we rely on directly
- features we expose through wrappers
- features where upstream docs are ambiguous or evolving

## Action Items

### Rust: resources and registration

- [ ] 1. Define `DbResource` and `ConnResource`, implement `rustler::Resource` for both, and register them in `on_load`.
- [ ] 2. Keep `rustler::init!("Elixir.TursoEx.Native", load = on_load)` as-is apart from the module name. Rustler 0.37 auto-discovers `#[rustler::nif]` functions.

### Rust: shared helpers

- [ ] 3. Add small helpers for consistent tagged tuple returns and operation-scoped error messages.
- [ ] 4. Implement `decode_term_to_value(term: Term) -> Result<turso::Value, String>` with this decode order: `nil`, `i64`, `f64`, `bool -> Integer(0/1)`, `String`, `{:blob, binary}`, raw `Binary`.
- [ ] 5. Implement Rust-owned query result types (`OwnedValue`, `QueryResult`) so async row collection does not depend on Elixir `Env`.
- [ ] 6. Implement row collection that reads `turso::Rows`, captures `rows.column_names()`, converts each `row.get_value(i)` into `OwnedValue`, and returns `QueryResult`.

### Rust: NIF implementations

- [ ] 7. Implement `db_open(path, sync_url, auth_token)` using `RUNTIME.block_on(async { turso::Builder::new_local(&path).build().await })`. Ignore sync args for now.
- [ ] 8. Implement `db_connect(db_ref)` using `db.connect()`.
- [ ] 9. Implement `conn_execute(conn_ref, sql, params)` using `conn.execute(&sql, decoded_params).await`.
- [ ] 10. Implement `conn_query(conn_ref, sql, params)` using `conn.query(&sql, decoded_params).await` plus the owned row collector.
- [ ] 11. Remove placeholder `db_push` and `db_pull` stubs from the Rust NIF for this phase.

### Elixir wrapper

- [ ] 12. Rename the low-level wrapper module to `TursoEx.Native` or introduce it alongside the existing file, so the public API does not advertise NIF implementation details.
- [ ] 13. Implement the first real public facade in [packages/turso_ex/lib/turso_ex.ex](packages/turso_ex/lib/turso_ex.ex):
  - `open/1`
  - `query/3`
  - `execute/3`
  - `one/3`
- [ ] 14. Add `%TursoEx.Conn{}`, `%TursoEx.Result{}`, and `%TursoEx.Error{}` modules.
- [ ] 15. Normalize native tagged tuples into public result/error structs at the facade layer.
- [ ] 16. Do not add `TursoEx.Driver` in Phase 2 unless the facade starts accumulating real SDK-parity glue that justifies the extra layer.
- [ ] 17. Remove `Ecto`, `EctoSQL`, and `DBConnection` deps from `packages/turso_ex` so the core package boundary matches the design.

### Documentation

Documentation work should follow working code and a passing test loop for the corresponding behavior. Do not block core implementation on polishing support docs before `conn_query/3`, `query/3`, and `one/3` are proven.

- [ ] 18. Keep [docs/architecture/nif-phase-2-flow.md](docs/architecture/nif-phase-2-flow.md) aligned with the real resource and data-flow design.
- [ ] 19. Maintain [docs/reference/turso-support-surface.md](docs/reference/turso-support-surface.md) as the canonical support document:
  - distinguish engine compatibility from wrapper API support
  - link upstream `COMPAT.md`
  - list upstream constraints and ambiguous areas
  - show which Turso Rust APIs are exposed in each phase
- [ ] 20. Add a usage-oriented doc for the public facade with the canonical examples: open, query, execute, one.
- [ ] 21. Document clearly that `transaction/2` is a later public API target, not a Phase 2 feature.
- [ ] 22. Document clearly that SQL engine features like UUID, JSON, vector, and Turso FTS are primarily accessed through SQL, not bespoke NIFs.
- [ ] 23. Document the mono-repo packaging rule explicitly:
  - core package lives in `packages/turso_ex`
  - adapter package is `ecto_turso_ex`
  - do not convert the repo into an umbrella
- [ ] 24. Document the Phase 2 data contract explicitly:
  - `%TursoEx.Conn{}` is opaque
  - `%TursoEx.Result{}` uses positional rows
  - `%TursoEx.Error{}` uses a stable small `kind` taxonomy
  - positional params are the public default
  - named params are out of scope unless intentionally added later
- [ ] 25. Add a short non-goals section to the public docs:
  - not an ORM
  - not a DBConnection adapter yet
  - not a schema-aware type-casting layer
  - not a promise of full SQLite feature emulation beyond upstream Turso behavior
- [ ] 26. State the upstream coupling policy clearly: this library targets the checked-in `turso` crate version and support claims are tied to that version.
- [ ] 27. Document connection semantics carefully:
  - users may pass `%TursoEx.Conn{}` between BEAM processes
  - this does not override upstream engine limitations around multi-process access or concurrency behavior

### Tests

- [ ] 28. Replace the placeholder test in [packages/turso_ex/test/turso_ex_test.exs](packages/turso_ex/test/turso_ex_test.exs) with public-API-focused tests.
- [ ] 29. Add native tests in `test/turso_ex/native_test.exs` or similar, covering:
  - open in-memory DB and get a resource ref
  - connect and get a connection ref
  - create a table through `conn_execute/3`
  - insert with params and verify affected row count
  - query rows and verify `columns`, `rows`, and `num_rows`
  - type round-trips for `nil`, integer, float, text, blob
  - boolean input encodes to `0/1` on query or insert
  - invalid SQL returns `{:error, reason}` with a binary reason
  - multiple inserts plus count query
- [ ] 30. Add public API tests covering:
  - `open/1`
  - `query/3` returns `%TursoEx.Result{}`
  - `execute/3` returns affected count
  - `one/3` returns scalar or `nil`
  - `%TursoEx.Conn{}` is the capability users pass around
  - public errors are normalized into `%TursoEx.Error{}`
- [ ] 31. Add public type-behavior tests covering:
  - `%TursoEx.Conn{}` cannot be meaningfully user-constructed
  - `%TursoEx.Result{}` preserves positional row order
  - `%TursoEx.Error.kind` falls into the documented taxonomy
  - `Inspect` output for public structs is useful and does not leak internals

Test notes:

- do not assert exact affected row count for `CREATE TABLE`; only assert success
- do assert exact affected row counts for inserts and updates where Turso guarantees them
- do not treat bool readback as boolean at the raw NIF layer
- add a small SQL compatibility smoke set from upstream docs, for example:
  - `RETURNING`
  - supported `PRAGMA table_info`
  - JSON function call
  - UUID function call
  - one Turso FTS probe once the local engine path can support it
- add explicit tests for documented unsupported behavior only when the wrapper needs to guard or explain it

### Verification

- [ ] 32. Run the targeted tests for the touched files.
- [ ] 33. Run `mix test` as the final confidence check.
- [ ] 34. Run a short `iex -S mix` manual flow using the public API:
  - open `:memory:`
  - create table
  - insert row
  - query row
- [ ] 35. Before later phases, build a small API inventory checklist from the `turso` crate public methods so support claims stay grounded in the actual Rust SDK version in use.

## Clarifications

> **Q:** Should we copy `ecto_libsql`'s registry-based connection design?
> **A:** No. `ecto_libsql` has different constraints. For this crate, `ResourceArc` is simpler and a better fit.

> **Q:** Should `conn_execute/3` auto-detect `SELECT`, `PRAGMA`, or `RETURNING` and secretly route to `query()`?
> **A:** No, not in Phase 2. That creates an ambiguous contract. Save SQL classification for the later adapter layer that actually needs it.

> **Q:** Should query results be built directly as Elixir terms during row iteration?
> **A:** No. Collect Rust-owned data first, then encode once at the NIF boundary.

> **Q:** Can raw NIF results preserve a semantic distinction between SQL text and blob values in Elixir terms?
> **A:** Not reliably. Both become Elixir binaries. Schema-aware decoding belongs in the adapter layer, not this phase.

> **Q:** Does "support the entire API from Turso" mean we need a bespoke NIF for every row in `COMPAT.md`?
> **A:** No. `COMPAT.md` is mostly engine compatibility. Many of those features are already available once SQL passes through correctly. The explicit NIF roadmap should target the public Rust SDK surface, with docs explaining which engine features are reachable through SQL.

> **Q:** Are savepoints safe to promise as supported?
> **A:** Not yet. Upstream docs are inconsistent here, and the current Rust transaction tests explicitly note a savepoint limitation. Document it as a caveat and verify before exposing any savepoint-oriented API.

> **Q:** Should the library lead with the NIF-shaped API?
> **A:** No. The NIF API is an internal seam. The library should lead with `TursoEx.open/query/execute/one/transaction` and keep the lower-level functions under `TursoEx.Native` or `TursoEx.Driver`.
