require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_link_header
def set_link_header(req)
  url = req.param('url')
  BenchmarkResponse.new(200, 'ok', { 'Link' => "<#{url}>; rel=\"canonical\"" }) # vuln-code-snippet vuln-line ruby_headerinj_link_header
end
# vuln-code-snippet end ruby_headerinj_link_header
