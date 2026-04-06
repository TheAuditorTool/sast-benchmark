require_relative 'shared'

# vuln-code-snippet start ruby_csrf_origin_check
def update_with_origin(req)
  origin = req.header('Origin')
  return BenchmarkResponse.error('CSRF rejected', 403) unless origin == 'https://example.com' # vuln-code-snippet safe-line ruby_csrf_origin_check
  BenchmarkResponse.ok('updated')
end
# vuln-code-snippet end ruby_csrf_origin_check
