require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_strip_crlf
def redirect_to_url_sanitized(req)
  input = req.param('url')
  safe_value = input.gsub(/[\r\n]/, '')  # vuln-code-snippet safe-line ruby_headerinj_strip_crlf
  BenchmarkResponse.new(302, '', { 'Location' => safe_value })
end
# vuln-code-snippet end ruby_headerinj_strip_crlf
