# TursoEx

Direct Elixir bindings for Turso, with a small public API and a native Rust NIF underneath.

This package is the core library in the mono-repo. It should stay focused on the direct SQL client API, not on Ecto adapter concerns.

## Why this exists

The motivating idea is simple: for LiveView and other BEAM applications, low-latency local data access is part of the user experience.

LiveView runs stateful server processes that receive events and push diffs back to the browser. Turso's local-first model is built around local SQLite files and embedded replicas with zero-network-latency reads. Put those together and you get a very interesting target shape for Elixir:

- local storage next to the application node
- fast reads in the UI loop
- a path to sync with a remote primary later
- one clean Elixir API instead of dropping straight into a foreign SDK

This package is building the Elixir-native version of that idea.

Planned public shape:

- `TursoEx.open/1`
- `TursoEx.query/3`
- `TursoEx.execute/3`
- `TursoEx.one/3`

Installation:

```elixir
def deps do
  [
    {:turso_ex, "~> 0.1.0"}
  ]
end
```
