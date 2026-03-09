defmodule TursoEx do
  @moduledoc """
  Public API for `turso_ex`.

  Phase 2 is still in progress, so the public entrypoints exist now as
  intentional stubs rather than pretending the package is just `hello/0`.
  """

  @doc """
  Open a local database and return a ready-to-use connection capability.
  """
  def open(_path_or_opts), do: not_implemented(:open)

  @doc """
  Execute a query and return a `%TursoEx.Result{}`.
  """
  def query(_conn, _sql, _params \\ []), do: not_implemented(:query)

  @doc """
  Execute a write statement and return the affected row count.
  """
  def execute(_conn, _sql, _params \\ []), do: not_implemented(:execute)

  @doc """
  Return a single scalar value or `nil`.
  """
  def one(_conn, _sql, _params \\ []), do: not_implemented(:one)

  defp not_implemented(function_name) do
    raise RuntimeError,
          "#{function_name}/#{arity_for(function_name)} is part of the planned public API but is not implemented yet"
  end

  defp arity_for(:open), do: 1
  defp arity_for(:query), do: 3
  defp arity_for(:execute), do: 3
  defp arity_for(:one), do: 3
end
