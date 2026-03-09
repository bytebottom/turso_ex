defmodule TursoEx.Result do
  @moduledoc """
  Positional query result returned by the public API.
  """

  defstruct columns: [], rows: [], num_rows: 0

  @type t :: %__MODULE__{
          columns: [String.t()],
          rows: [[term()]],
          num_rows: non_neg_integer()
        }
end
