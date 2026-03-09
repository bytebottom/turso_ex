# Plan 03: Elixir Facade

This plan covers the Elixir-facing surface in `packages/turso_ex`.

## Goal

Expose a clean public API on top of the native layer without leaking NIF-shaped semantics into the main library.

## Rules

- keep `TursoEx.Native` low-visibility
- do not add `TursoEx.Driver` in Phase 2 unless it clearly removes real complexity
- normalize native errors into public `%TursoEx.Error{}`
- keep the public story centered on `open`, `query`, `execute`, and `one`

## Action Items

- [ ] 1. Keep `TursoEx.Native` aligned with the Rust module name and low-level role.
- [ ] 2. Implement the public facade in `packages/turso_ex/lib/turso_ex.ex`.
- [ ] 3. Add `open/1`.
- [ ] 4. Add `query/3`.
- [ ] 5. Add `execute/3`.
- [ ] 6. Add `one/3`.
- [ ] 7. Add `%TursoEx.Conn{}`.
- [ ] 8. Add `%TursoEx.Result{}`.
- [ ] 9. Add `%TursoEx.Error{}`.
- [ ] 10. Normalize Rust-side tagged tuples into public results and `%TursoEx.Error{}`.
- [ ] 11. Keep `packages/turso_ex` free of `Ecto`, `EctoSQL`, and `DBConnection`.

## Facade Contracts

- `open/1` accepts either a bare path string or `path: ...`
- `open/1` calls `db_open/1` and `db_connect/1`, then returns a connection capability
- `%TursoEx.Conn{}` retains both handles internally so later sync work does not force a public struct break
- `query/3` returns `%TursoEx.Result{}`
- `execute/3` returns affected row count
- `one/3` returns `{:ok, nil}` for zero rows
- `one/3` returns `{:ok, value}` for exactly one row and one column
- `one/3` returns an error for multiple rows or multiple columns

## Exit Criteria

- a caller can use the public API without touching native implementation details
- the public structs exist and have stable shape
- the package still reads like an Elixir library, not a wrapped foreign interface
