require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_strip_crlf2
def set_custom_stripped(req)
  val = req.param('value').gsub(/[\r\n\x00]/, '') # vuln-code-snippet safe-line ruby_headerinj_strip_crlf2
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Custom'] = val
  response
end
# vuln-code-snippet end ruby_headerinj_strip_crlf2
