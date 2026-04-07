require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_forwarded_host
def echo_forwarded_host(req)
  host = req.header('HTTP_HOST')
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Forwarded-Host'] = host # vuln-code-snippet vuln-line ruby_headerinj_forwarded_host
  response
end
# vuln-code-snippet end ruby_headerinj_forwarded_host
