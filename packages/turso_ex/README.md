# TursoEx

Direct Elixir bindings for Turso, with a small public API and a native Rust NIF underneath.

This package is the core library in the mono-repo. It should stay focused on the direct SQL client API, not on Ecto adapter concerns.

## Why this exists

Data latency is UI latency.

For LiveView applications, that is not a slogan. It is the whole game.

Deploying application nodes close to users was always fairly straightforward. Getting the data path close to users as well was the hard part. That gap matters a lot for LiveView, because user events are handled on the server and often need fresh data before the next diff can be rendered.

Turso's local-first model is built around local SQLite files and embedded replicas with zero-network-latency reads. Put those pieces together and you get a much more attractive shape for Elixir:

- application nodes near users
- data reads near those application nodes too
- faster interactions in the LiveView event loop
- a path to sync with a remote primary later
- one clean Elixir API instead of dropping straight into a foreign SDK

This package is the Elixir-native version of that idea.

Planned public shape:

- `TursoEx.open/1`
- `TursoEx.query/3`
- `TursoEx.execute/3`
- `TursoEx.one/3`

Current status:

- the public entrypoints exist as explicit stubs
- Phase 2 implementation is still in progress

Installation:

```elixir
def deps do
  [
    {:turso_ex, "~> 0.1.0"}
  ]
end
```
