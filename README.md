# TursoEx Mono-Repo

TursoEx exists because the combination of Turso and LiveView is unusually compelling.

Phoenix LiveView keeps UI state on the server, handles events in a LiveView process, and pushes diffs back to the browser over a persistent connection. That model is excellent, but it makes request-to-database latency matter a lot. If every click turns into a server event and a database round trip, the database is part of the UI loop.

Turso is interesting here because its local-first story is built around SQLite files, embedded replicas, zero-network-latency local reads, and sync back to a remote primary. For BEAM applications, especially LiveView apps, that opens up a very attractive shape: keep storage physically close to the application process, optimize for local latency, and keep a cloud-connected path when you need it.

That is the bet behind this repo.

Phase 2 is only the local foundation, not the whole end-state. The point is to build the clean Elixir API and native boundary first, so local-only usage is solid before remote sync and fuller Turso parity arrive.

Why this is exciting for Elixir and LiveView:

- LiveView processes receive events and render diffs from the server, so lower database latency directly improves the feel of interactive screens.
- Turso's embedded-replica model gives you a local SQLite file with zero-network-latency reads and explicit sync semantics.
- BEAM apps already like keeping state and coordination close to the process that owns the work. A local database file on the same node fits that instinct well.
- This should be especially interesting for LiveView-heavy apps where the fastest path is often "handle event, read local state, render diff".

Relevant sources:

- Phoenix LiveView overview: <https://hexdocs.pm/phoenix/live_view.html>
- Phoenix LiveView latency notes: <https://hexdocs.pm/phoenix_live_view/1.1.1/js-interop.html>
- Turso local-first overview: <https://turso.tech/local-first>
- Turso embedded replicas: <https://docs.turso.tech/features/embedded-replicas>

This repository is intended to stay as a mono-repo without becoming an umbrella:

- the core `turso_ex` package lives in `packages/turso_ex`
- the `ecto_turso_ex` adapter package lives in `packages/ecto_turso_ex`
- the Rust NIF crate lives in `packages/turso_ex/native/turso_nif`

That keeps development in one place without letting Ecto concerns leak into the core package too early.

## Repository Map

```text
.
├── AGENTS.md
├── PLAN.md
├── README.md
├── docs/
│   ├── README.md
│   ├── architecture/
│   └── reference/
├── packages/
│   ├── ecto_turso_ex/
│   └── turso_ex/
│       └── native/turso_nif/
└── Cargo.toml
```

What lives where:

- `packages/turso_ex/`
  Core `turso_ex` package.
- `packages/ecto_turso_ex/`
  Separate Mix project for the future Ecto adapter.
- `packages/turso_ex/native/turso_nif/`
  Rust NIF crate owned by the core package.
- `Cargo.toml`
  Rust workspace manifest for the NIF crate.
- `packages/*/mix.exs`
  Elixir package manifests for the workspace packages.
- `docs/architecture/`
  Diagrams and flow docs.
- `docs/reference/`
  Support contracts and compatibility notes.
- `PLAN.md`
  Working implementation plan, not general reference documentation.

Package-specific docs live in:

- [packages/turso_ex/README.md](packages/turso_ex/README.md)
- [packages/ecto_turso_ex/README.md](packages/ecto_turso_ex/README.md)

## Working In The Repo

Run package commands from the package you are changing:

- core package:
  `cd packages/turso_ex && mix test`
- adapter package:
  `cd packages/ecto_turso_ex && mix test`
