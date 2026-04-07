require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_pragma_inject
def set_pragma(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Pragma'] = "no-cache, #{req.param('directive')}" # vuln-code-snippet vuln-line ruby_headerinj_pragma_inject
  response
end
# vuln-code-snippet end ruby_headerinj_pragma_inject
