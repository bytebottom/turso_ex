# Plan 04: Tests And Verification

This plan proves the Phase 2 behavior.

Do not treat this as cleanup. This is where the contract becomes real.

This plan runs alongside plans 02 and 03:

- start native tests as soon as native behavior exists
- start public API tests as soon as facade behavior exists
- use the final verification steps only after both layers are in place

## Test Layers

### Native tests

- [ ] 1. Open an in-memory DB and get a resource ref.
- [ ] 2. Connect and get a connection ref.
- [ ] 3. Create a table through `conn_execute/3`.
- [ ] 4. Insert with params and verify affected row count.
- [ ] 5. Query rows and verify `columns`, `rows`, and `num_rows`.
- [ ] 6. Cover `nil`, integer, float, text, and blob.
- [ ] 7. Verify boolean input encodes to `0/1`.
- [ ] 8. Verify invalid SQL returns `{:error, reason}`.
- [ ] 9. Verify constraint violations return `{:error, reason}`.
- [ ] 10. Verify bad resource refs fail cleanly.
- [ ] 11. Verify invalid parameter shapes fail cleanly.

### Public API tests

- [ ] 12. Replace the placeholder public test.
- [ ] 13. Verify `open/1` with bare string and `path: ...`.
- [ ] 14. Verify `query/3` returns `%TursoEx.Result{}`.
- [ ] 15. Verify `execute/3` returns affected count.
- [ ] 16. Verify `one/3` returns `{:ok, nil}` for zero rows.
- [ ] 17. Verify `one/3` returns scalar for exactly one row and one column.
- [ ] 18. Verify `one/3` errors on multiple rows.
- [ ] 19. Verify `one/3` errors on multiple columns.
- [ ] 20. Verify public errors normalize into `%TursoEx.Error{}`.

### Type-behavior tests

- [ ] 21. Verify invalid user-constructed `%TursoEx.Conn{}` values fail cleanly.
- [ ] 22. Verify `%TursoEx.Result{}` preserves positional row order.
- [ ] 23. Verify `%TursoEx.Error.kind` stays within the documented taxonomy.
- [ ] 24. Verify `Inspect` output is useful and does not leak internals.

## Verification

- [ ] 25. Run targeted tests for touched files.
- [ ] 26. Run `mix test` in `packages/turso_ex`.
- [ ] 27. Run a short `iex -S mix` manual flow:
  - open `:memory:`
  - create table
  - insert row
  - query row

## Test Notes

- do not assert exact affected row count for `CREATE TABLE`
- do assert exact affected row counts for inserts and updates where guaranteed
- do not treat bool readback as boolean at the raw NIF layer
- add a small SQL compatibility smoke set when the local path supports it

## Exit Criteria

- the native contract is proven
- the public contract is proven
- the docs can start describing behavior without bluffing
