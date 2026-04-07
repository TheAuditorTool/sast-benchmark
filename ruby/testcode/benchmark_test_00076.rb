require_relative 'shared'
require 'socket'

def fetch_socket(req)
  host = req.param('host')
  port = req.param('port').to_i
  Socket.tcp(host, port) { |s| s.read(1024) }
  BenchmarkResponse.json({ ok: true })
end
