require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_rack_protection_crlf
def middleware_strip_crlf(req)
  val = req.header('HTTP_X_CUSTOM').to_s.gsub(/[\r\n]/, '') # vuln-code-snippet safe-line ruby_headerinj_rack_protection_crlf
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Custom'] = val
  response
end
# vuln-code-snippet end ruby_headerinj_rack_protection_crlf
