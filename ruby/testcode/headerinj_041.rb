require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_no_user_in_header
def set_constant_headers(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Type'] = 'application/json' # vuln-code-snippet safe-line ruby_headerinj_no_user_in_header
  response.headers['X-Content-Type-Options'] = 'nosniff'
  response
end
# vuln-code-snippet end ruby_headerinj_no_user_in_header
