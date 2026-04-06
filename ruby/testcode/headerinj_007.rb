require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_content_type
def set_content_type(req)
  ctype = req.param('type')
  BenchmarkResponse.new(200, 'ok', { 'Content-Type' => ctype }) # vuln-code-snippet vuln-line ruby_headerinj_content_type
end
# vuln-code-snippet end ruby_headerinj_content_type
