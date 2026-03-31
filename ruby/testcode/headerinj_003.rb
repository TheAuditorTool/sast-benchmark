require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_custom_header
def proxy_request(req)
  forwarded = req.header('X-Forward')
  headers = { 'Content-Type' => 'text/plain' }
  headers['X-User-Data'] = forwarded  # vuln-code-snippet vuln-line ruby_headerinj_custom_header
  BenchmarkResponse.new(200, 'ok', headers)
end
# vuln-code-snippet end ruby_headerinj_custom_header
