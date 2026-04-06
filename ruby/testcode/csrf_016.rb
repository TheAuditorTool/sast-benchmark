require_relative 'shared'

# vuln-code-snippet start ruby_csrf_samesite_csrf
def update_samesite(req)
  cookie_token = req.cookie('csrf_token')
  param_token = req.post('csrf_token')
  return BenchmarkResponse.error('CSRF rejected', 403) unless cookie_token && param_token && cookie_token == param_token # vuln-code-snippet safe-line ruby_csrf_samesite_csrf
  BenchmarkResponse.ok('updated')
end
# vuln-code-snippet end ruby_csrf_samesite_csrf
