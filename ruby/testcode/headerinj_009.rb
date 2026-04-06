require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_x_forwarded
def proxy_forward(req)
  client_ip = req.param('ip')
  BenchmarkResponse.new(200, 'ok', { 'X-Forwarded-For' => client_ip }) # vuln-code-snippet vuln-line ruby_headerinj_x_forwarded
end
# vuln-code-snippet end ruby_headerinj_x_forwarded
