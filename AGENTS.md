# TursoEx Project Instructions

This file defines project-level guardrails for `turso_ex`.

These rules are here to protect the end-state shape of the library:

- a tiny public API
- clean package boundaries
- honest documentation
- no Ecto-driven contamination of the core package

## Product Bar

Build `turso_ex` as an Elixir library first, and a NIF wrapper second.

If a change makes the public API feel like renamed FFI, it is probably the wrong change.

The public API should stay centered on:

- `TursoEx.open/1`
- `TursoEx.query/3`
- `TursoEx.execute/3`
- `TursoEx.one/3`
- `TursoEx.transaction/2` later, when it is real

The public data model should stay centered on:

- `%TursoEx.Conn{}`
- `%TursoEx.Result{}`
- `%TursoEx.Error{}`

Prefer a small, boring, stable surface over broad early parity.

## Package Boundaries

Keep this repository as a mono-repo with separate packages, not an umbrella.

Rules:

- the repo root is a workspace, not a Mix app
- the core package lives in `packages/turso_ex`
- the adapter package lives in `packages/ecto_turso_ex`
- the Rust NIF crate lives in `packages/turso_ex/native/turso_nif`
- `packages/turso_ex` must not depend on `Ecto`, `EctoSQL`, or `DBConnection`
- do not restructure this repo into an umbrella unless there is a deployment reason, not just development convenience

The Ecto adapter exists to depend on the core package, not to dictate the shape of the core package.

## Layering Rules

Prefer this layering:

- `TursoEx`, canonical public API
- `TursoEx.Native`, thin NIF boundary
- `TursoEx.Driver`, only if later parity work clearly justifies it

Rules:

- do not expose `TursoEx.Native` as the primary user-facing API
- do not add `TursoEx.Driver` preemptively
- do not add a new layer unless it removes real complexity
- keep the public API shaped around user intent, not upstream method names

## API Taste

Write APIs the way a good Elixir library should feel:

- one obvious happy path
- stable structs instead of ad hoc maps and tuples everywhere
- minimal ceremony
- advanced control without infecting the beginner path
- no hidden SQL routing tricks in the core public contract

Specific rules:

- `open/1` returns a ready-to-use connection capability
- `%TursoEx.Conn{}` should stay opaque and boring
- `%TursoEx.Result{}` should use positional rows in the base API
- `%TursoEx.Error{}` should stay machine-meaningful and pleasant in `iex`
- avoid adding convenience APIs unless they clearly earn their keep

## Documentation Rules

Documentation must tell the truth, especially around limitations.

Rules:

- distinguish engine compatibility, Rust SDK parity, and Elixir API support
- do not claim wrapper support just because the engine supports a SQL feature
- do not imply SQLite parity where upstream Turso does not provide it
- document savepoint, FTS, extension, and concurrency caveats honestly
- tie support claims to the checked-in `turso` crate version

Write docs after the corresponding behavior works and tests pass. Do not let doc polish block the implementation loop.

Repository doc layout:

- put diagrams and flow docs in `docs/architecture/`
- put contracts, compatibility notes, and support surfaces in `docs/reference/`
- keep the working plan set in `plans/`, not in a root `PLAN.md`
- do not dump every markdown file into `docs/` root without a reason

## Testing Rules

For this project, clean design claims must be backed by tests.

Rules:

- test the public API, not just the NIF seam
- add focused native tests for boundary behavior and type conversion
- add public tests for `%TursoEx.Conn{}`, `%TursoEx.Result{}`, and `%TursoEx.Error{}`
- verify errors are structured and stable enough to build on
- verify docs-critical claims before documenting them as supported

## Rust / NIF Rules

NIFs run inside the BEAM VM. A panic or segfault in Rust kills the entire Erlang node, not just the calling process.

Rules:

- never panic: catch all `Result`/`Option` failures and return `Err(rustler::Error)` instead
- never `unwrap()` or `expect()` on fallible paths that touch user data or I/O
- schedule every NIF with `#[rustler::nif(schedule = "DirtyIo")]` so the BEAM scheduler stays responsive
- never hold a `rustler::Env` or build `rustler::Term` values across a `.await` point; collect Rust-owned data first, encode to terms once after the async work completes
- use `ResourceArc<T>` for opaque handles (`Database`, `Connection`); do not serialize handles to strings or IDs
- prefix NIF error atoms/strings with the operation name so Elixir callers can pattern-match (`"open: ..."`, `"query: ..."`)
- do not allocate unbounded memory inside a NIF; cap row buffers or stream results if a query could return arbitrarily many rows
- wrap the turso async API with `RUNTIME.block_on(...)` inside DirtyIo NIFs; do not spawn detached Tokio tasks that outlive the NIF call
- if a Rust dependency introduces `unsafe`, audit it before merging

## Smell List

These are warning signs:

- adding Ecto dependencies to `packages/turso_ex`
- adding public `db_*` or `conn_*` functions because they are easy to expose
- introducing `Driver` before real parity work proves the need
- documenting features before the code path works
- broadening the public API to mirror the Rust crate mechanically
- hiding ambiguity behind “smart” behavior instead of making the contract explicit

If a proposed change hits one of these, slow down and justify it clearly before proceeding.
