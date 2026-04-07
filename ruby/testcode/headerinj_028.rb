require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_content_length
def set_content_length(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Length'] = req.param('size') # vuln-code-snippet vuln-line ruby_headerinj_content_length
  response
end
# vuln-code-snippet end ruby_headerinj_content_length
