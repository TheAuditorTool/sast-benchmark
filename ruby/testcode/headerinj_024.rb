require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_content_security
def set_csp(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Security-Policy'] = "script-src #{req.param('domain')}" # vuln-code-snippet vuln-line ruby_headerinj_content_security
  response
end
# vuln-code-snippet end ruby_headerinj_content_security
