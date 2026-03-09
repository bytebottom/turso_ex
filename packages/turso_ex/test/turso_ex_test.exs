defmodule TursoExTest do
  use ExUnit.Case

  test "public API functions exist as explicit not implemented stubs" do
    assert_raise RuntimeError, ~r/open\/1/, fn ->
      TursoEx.open(":memory:")
    end

    assert_raise RuntimeError, ~r/query\/3/, fn ->
      TursoEx.query(%TursoEx.Conn{db: :placeholder, conn: :placeholder}, "select 1", [])
    end

    assert_raise RuntimeError, ~r/execute\/3/, fn ->
      TursoEx.execute(%TursoEx.Conn{db: :placeholder, conn: :placeholder}, "select 1", [])
    end

    assert_raise RuntimeError, ~r/one\/3/, fn ->
      TursoEx.one(%TursoEx.Conn{db: :placeholder, conn: :placeholder}, "select 1", [])
    end
  end
end
