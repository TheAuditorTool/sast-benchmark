require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_sanitize_crlf
def redirect_safe(req)
  url = req.param('url')
  safe_url = url.delete("\r\n") # vuln-code-snippet safe-line ruby_headerinj_sanitize_crlf
  BenchmarkResponse.new(302, '', { 'Location' => safe_url })
end
# vuln-code-snippet end ruby_headerinj_sanitize_crlf
