defmodule TursoEx.Native do
  use Rustler, otp_app: :turso_ex, crate: :turso_nif, path: "native/turso_nif"

  def db_open(_path, _sync_url, _auth_token), do: :erlang.nif_error(:nif_not_loaded)
  def db_connect(_db), do: :erlang.nif_error(:nif_not_loaded)
  def conn_execute(_conn, _sql, _params), do: :erlang.nif_error(:nif_not_loaded)
  def conn_query(_conn, _sql, _params), do: :erlang.nif_error(:nif_not_loaded)
  def db_push(_db), do: :erlang.nif_error(:nif_not_loaded)
  def db_pull(_db), do: :erlang.nif_error(:nif_not_loaded)
end
