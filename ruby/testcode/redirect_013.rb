require_relative 'shared'

# vuln-code-snippet start ruby_redirect_header_raw
def redirect_via_header(req)
  url = req.param('url')
  BenchmarkResponse.new(302, '', { 'Location' => url }) # vuln-code-snippet vuln-line ruby_redirect_header_raw
end
# vuln-code-snippet end ruby_redirect_header_raw
