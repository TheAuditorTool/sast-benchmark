require_relative 'shared'

FIXED_CSP = "default-src 'self'; script-src 'none'".freeze

# vuln-code-snippet start ruby_headerinj_constant_csp
def set_constant_csp(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Security-Policy'] = FIXED_CSP # vuln-code-snippet safe-line ruby_headerinj_constant_csp
  response
end
# vuln-code-snippet end ruby_headerinj_constant_csp
