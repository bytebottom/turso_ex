# Plan Index

This repo now uses a plan set instead of one giant plan file.

The point is simple: each plan should describe one stage of work clearly enough that it is obvious what to do next, what is blocked, and what "done" means.

## Execution Order

1. [plans/01-contract.md](plans/01-contract.md)
   The product bar, package boundaries, public contract, and Phase 2 scope.
2. [plans/02-native-nif-layer.md](plans/02-native-nif-layer.md)
   Rust resources, decoding, row collection, and NIF implementation work.
3. [plans/03-elixir-facade.md](plans/03-elixir-facade.md)
   `TursoEx.Native`, the public facade, and public structs.
4. [plans/04-tests-and-verification.md](plans/04-tests-and-verification.md)
   Parallel test work and final verification.
5. [plans/05-docs-and-support-surface.md](plans/05-docs-and-support-surface.md)
   Documentation work that should happen after the code and tests prove the behavior.
6. [plans/06-future-roadmap.md](plans/06-future-roadmap.md)
   Later phases for SDK parity, transactions, sync, and compatibility probes.

## Order Rules

- Treat `01` as a contract, not a checklist.
- Start native tests as soon as `02` produces real behavior.
- Start public API tests as soon as `03` produces real behavior.
- Do not start the docs plan until the code path and tests for that behavior are real.
- Do not let the adapter package drive the core package API.
- Do not broaden the public API just because the Rust SDK has more methods.
- Keep the public story centered on `open`, `query`, `execute`, and `one`.

## Core Thesis

The product is the Elixir API, not the NIF seam.

The bar is:

- one obvious happy path
- stable public structs
- honest docs
- package boundaries that stay clean under pressure

If the public API feels like renamed FFI, the implementation is going off the rails.
