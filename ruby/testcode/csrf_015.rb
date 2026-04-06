require_relative 'shared'

# vuln-code-snippet start ruby_csrf_wrong_compare
def update_with_wrong_compare(req)
  token = req.post('csrf_token')
  if token # vuln-code-snippet vuln-line ruby_csrf_wrong_compare
    BenchmarkResponse.ok('updated')
  else
    BenchmarkResponse.error('CSRF rejected', 403)
  end
end
# vuln-code-snippet end ruby_csrf_wrong_compare
