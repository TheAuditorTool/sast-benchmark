require_relative 'shared'

# vuln-code-snippet start ruby_csrf_double_submit
def update_with_double_submit(req)
  cookie_token = req.cookie('csrf_token')
  header_token = req.header('X-CSRF-Token')
  return BenchmarkResponse.error('CSRF rejected', 403) unless cookie_token == header_token # vuln-code-snippet safe-line ruby_csrf_double_submit
  BenchmarkResponse.ok('updated')
end
# vuln-code-snippet end ruby_csrf_double_submit
