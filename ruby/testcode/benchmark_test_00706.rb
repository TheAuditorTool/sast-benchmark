require_relative 'shared'

FIXED_CSP = "default-src 'self'; script-src 'none'".freeze

def set_constant_csp(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Security-Policy'] = FIXED_CSP
  response
end
