require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_retry_after
def set_retry_after(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Retry-After'] = req.param('seconds') # vuln-code-snippet vuln-line ruby_headerinj_retry_after
  response
end
# vuln-code-snippet end ruby_headerinj_retry_after
