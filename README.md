# Turso for Elixir

> The way to make programs fast is not to optimize the programs but to give the programs less to do.<br />
> _— Joe Armstrong, *Coders at Work* (2009)_

LiveView keeps UI state on the server. Every event hits a process, reads some data, and pushes a diff back to the browser. That loop is fast when the database is close. It gets slow when every read crosses an ocean.

BEAM nodes are easy to deploy near users. The data has always been the hard part. TursoEx fixes that.

## How

[Turso](https://turso.tech) gives you a local SQLite file that serves reads with zero network latency and syncs back to a remote primary on your schedule. TursoEx puts that engine inside your BEAM node via Rust NIFs, so your LiveView process can read from a file on the same machine instead of round-tripping to a database across the wire.

`handle_event` -> local read -> render diff. The whole loop in microseconds.

## The API

TursoEx is an Elixir-first library. The public surface is designed around what you'd want to write, not what the Rust SDK happens to expose.

For local databases, `open/1` should accept either a bare string path or `path: ...`.

```elixir
{:ok, conn} = TursoEx.open(":memory:")
{:ok, conn} = TursoEx.open(path: "/tmp/my.db")

{:ok, _} = TursoEx.execute(conn, "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")
{:ok, _} = TursoEx.execute(conn, "INSERT INTO users (name) VALUES (?1)", ["Ada"])

{:ok, result} = TursoEx.query(conn, "SELECT * FROM users")
# %TursoEx.Result{columns: ["id", "name"], rows: [[1, "Ada"]]}
```

The public API stays small on purpose: `open`, `query`, `execute`, `one`. Ecto support lives in a separate adapter package that depends on the core, not the other way around.

## Why Turso over plain SQLite

Turso is a ground-up Rust rewrite of the SQLite engine with features that matter for production Elixir apps:

- **Embedded replicas** with explicit sync semantics
- **Vector search** built into the engine
- **Full-text search** powered by Tantivy (not the SQLite FTS extension)
- **Multi-tenancy** via per-database branching and schema sharing

All of it runs locally, in-process.

## Project status

Early. The NIF boundary compiles and the scaffold is in place, but real queries are not wired up yet. Phase 2 (local reads and writes through the full Elixir API) is the current focus.

## Roadmap

The project is moving in six visible steps:

- Phase 2, make the local client real, `open`, `query`, `execute`, `one`
- Phase 3, add the most important local SDK parity features, `batch`, `prepare`, `query_row`, metadata, pragmas
- Phase 4, add transactions without bloating the public API
- Phase 5, add remote and sync support in the shape Turso users already expect
- Phase 6, keep compatibility claims honest with targeted probes and support docs

The full plan set lives in [PLAN.md](PLAN.md) and [plans/06-future-roadmap.md](plans/06-future-roadmap.md).

### Official Turso Capabilities We Intend To Map

The official Turso bindings mostly converge on the same capability set:

- open local, in-memory, remote, and embedded replica databases
- query and execute SQL with positional and named parameters
- support explicit sync operations
- expose batch execution, prepared statements, metadata, and transactions

That should be visible in `turso_ex` over time. The Elixir API will stay Elixir-shaped, but it should still feel recognizably like a real Turso client.

Some ideas may still exist as future Elixir conveniences even when they do not map 1:1 to the local `turso` crate. Those need to be documented honestly as conveniences, not parity.

The likely end-state public story looks like this:

```elixir
{:ok, conn} = TursoEx.open(path: "/tmp/app.db")
{:ok, conn} = TursoEx.open(url: "libsql://...", auth_token: token)

{:ok, conn} =
  TursoEx.open(
    path: "/tmp/app.db",
    sync_url: "libsql://...",
    auth_token: token
  )

{:ok, result} = TursoEx.query(conn, "SELECT * FROM users WHERE id = ?1", [1])
{:ok, count} = TursoEx.execute(conn, "UPDATE users SET name = ?1 WHERE id = ?2", ["Ada", 1])
:ok = TursoEx.push(conn)
:ok = TursoEx.pull(conn)
```

### What Will Stay SQL-First

Some Turso features matter a lot, but they should still flow through ordinary SQL:

- vector search
- full-text search
- JSON functions
- most PRAGMA usage
- extension-backed SQL features

Those need good docs and examples, not a pile of wrapper functions.

## Repository layout

Mono-repo with two packages and one Rust crate:

```
packages/turso_ex/              Core library (no Ecto dependency)
packages/turso_ex/native/       Rust NIF crate (turso + rustler)
packages/ecto_turso_ex/         Ecto adapter (depends on turso_ex)
docs/                           Architecture diagrams, reference docs
```

Work happens inside the package you're changing:

```sh
cd packages/turso_ex && mix test       # core
cd packages/ecto_turso_ex && mix test  # adapter
```

## Acknowledgements

TursoEx would not exist without [ecto_libsql](https://github.com/ocean/ecto_libsql) by Drew Robinson. Its Rust NIF architecture, parameter encoding, and row decoding patterns were the blueprint for this project. If you need libSQL in Elixir today, start there.

## Links

- [Turso](https://turso.tech)
- [Embedded replicas](https://docs.turso.tech/features/embedded-replicas)
- [Phoenix LiveView](https://hexdocs.pm/phoenix_live_view)
- [ecto_libsql](https://github.com/ocean/ecto_libsql)
