require_relative 'shared'

FIXED_CONTENT_TYPE = 'application/json; charset=utf-8'.freeze

# vuln-code-snippet start ruby_headerinj_content_type_fixed
def set_fixed_content_type(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Type'] = FIXED_CONTENT_TYPE # vuln-code-snippet safe-line ruby_headerinj_content_type_fixed
  response
end
# vuln-code-snippet end ruby_headerinj_content_type_fixed
