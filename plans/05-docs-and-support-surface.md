# Plan 05: Docs And Support Surface

This plan happens after code and tests pass for the behavior being documented.

Do not let documentation work block the implementation loop.

## Goal

Document what is true, not what is merely planned.

## Action Items

- [ ] 1. Keep `docs/architecture/nif-phase-2-flow.md` aligned with the real resource and data-flow design.
- [ ] 2. Maintain `docs/reference/turso-support-surface.md` as the canonical support document.
- [ ] 3. Add a usage-oriented doc for the public facade with canonical examples.
- [ ] 4. Document clearly that `transaction/2` is a later target, not a Phase 2 feature.
- [ ] 5. Document clearly that UUID, JSON, vector, and Turso FTS are primarily accessed through SQL.
- [ ] 6. Document the mono-repo packaging rule explicitly.
- [ ] 7. Document the Phase 2 data contract explicitly.
- [ ] 8. Add a short non-goals section to the public docs.
- [ ] 9. State the upstream coupling policy clearly.
- [ ] 10. Document connection semantics carefully.

## Documentation Rules

- distinguish engine compatibility, Rust SDK parity, and Elixir wrapper support
- do not imply wrapper support just because the engine supports a SQL feature
- do not imply SQLite parity where upstream Turso does not provide it
- tie support claims to the checked-in `turso` crate version

## Upstream Constraints To Document Honestly

- concurrent sharing is possible, but concurrent writes may return busy errors
- `VACUUM` unsupported
- SQLite FTS3/FTS4/FTS5 unsupported
- dynamic SQLite loadable extensions unsupported
- savepoint support ambiguous until verified

## Exit Criteria

- a new reader can tell what is implemented, what is planned, and what is unsupported
- docs no longer blur engine support with wrapper support
