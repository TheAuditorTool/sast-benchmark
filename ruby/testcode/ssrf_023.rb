require_relative 'shared'
require 'socket'

# vuln-code-snippet start ruby_ssrf_socket_tcp
def fetch_socket(req)
  host = req.param('host')
  port = req.param('port').to_i
  Socket.tcp(host, port) { |s| s.read(1024) } # vuln-code-snippet vuln-line ruby_ssrf_socket_tcp
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_socket_tcp
