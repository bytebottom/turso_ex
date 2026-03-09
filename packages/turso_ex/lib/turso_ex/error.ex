defmodule TursoEx.Error do
  @moduledoc """
  Public error shape for `turso_ex`.
  """

  @type kind ::
          :invalid_argument
          | :constraint
          | :sql_error
          | :busy
          | :misuse
          | :internal

  defstruct [:operation, :message, :kind, details: %{}]

  @type t :: %__MODULE__{
          operation: atom() | nil,
          message: String.t() | nil,
          kind: kind() | nil,
          details: map()
        }
end
