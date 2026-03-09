defmodule EctoTursoExTest do
  use ExUnit.Case, async: true

  test "package scaffold loads" do
    assert Code.ensure_loaded?(EctoTursoEx)
  end
end
