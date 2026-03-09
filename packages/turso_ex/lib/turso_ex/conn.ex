defmodule TursoEx.Conn do
  @moduledoc """
  Opaque connection capability for the public API.

  The struct shape is intentionally minimal for now. Later phases will keep both
  database and connection handles inside this struct without changing the public
  module name.
  """

  @enforce_keys [:db, :conn]
  defstruct [:db, :conn]

  @opaque t :: %__MODULE__{
            db: term(),
            conn: term()
          }
end
