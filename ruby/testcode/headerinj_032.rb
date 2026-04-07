require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_vary_inject
def set_vary(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Vary'] = "Accept-Encoding, #{req.param('header')}" # vuln-code-snippet vuln-line ruby_headerinj_vary_inject
  response
end
# vuln-code-snippet end ruby_headerinj_vary_inject
