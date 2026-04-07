require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_accept_ranges
def set_accept_ranges(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Accept-Ranges'] = req.param('unit') # vuln-code-snippet vuln-line ruby_headerinj_accept_ranges
  response
end
# vuln-code-snippet end ruby_headerinj_accept_ranges
