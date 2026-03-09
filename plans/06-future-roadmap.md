# Plan 06: Future Roadmap

This plan keeps the later phases visible without clogging Phase 2 execution.

## Why This Exists

`COMPAT.md` is not a to-do list of bespoke NIFs.

The roadmap needs to track three surfaces separately:

- engine compatibility
- Rust SDK parity
- the public Elixir wrapper

It also needs to reflect the common capability shape across the official Turso bindings.

But crate parity and cross-SDK aspirations are not the same thing.

That common shape is roughly:

- open local, in-memory, remote, and embedded replica databases
- query and execute with positional and named params
- explicit sync controls
- batch execution
- prepared statements and query-row helpers
- transactions with explicit behavior or mode

When the local `turso` crate does not expose a capability directly, the roadmap should say so plainly instead of silently treating platform docs as crate parity.

## Phase 3

Connection utility and statement parity:

- expand `open/1` with local builder options, encryption and experimental local features
- `execute_batch`
- `prepare`
- `prepare_cached`
- `query_row`
- pragma helpers
- connection metadata helpers
- named parameters
- `TursoEx.Driver` only if needed

## Phase 4

Transaction parity:

- public `transaction/2`
- transaction behavior or mode support
- transaction begin / commit / rollback / finish
- honest savepoint documentation

## Phase 5

Sync and remote parity:

- full remote builder path
- embedded replica options
- `push`
- `pull`
- `checkpoint`
- `stats`
- auth, URL, bootstrap, long poll, encryption, and partial-sync options
- builder-level experimental feature flags where justified, `attach`, `triggers`, `materialized_views`

## Phase 6

Compatibility probes:

- important SQL statements and PRAGMAs
- JSON, UUID, vector, time, percentile functions
- Turso FTS behavior
- strict tables and extension notes
- cross-check public support claims against official SDK docs and upstream compatibility docs

## Standing Rules

- do not add a bespoke NIF for SQL features that already flow through `query/execute`
- do not let later parity work bloat the public API mechanically
- document ambiguous upstream behavior before promising support
- do not invent a dedicated `search` API when the feature is really SQL
- do not invent a dedicated vector API unless SQL proves insufficient in practice
