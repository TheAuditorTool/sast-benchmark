require_relative 'shared'

# vuln-code-snippet start ruby_xss_link_to_href
def render_back_link(req)
  url = req.param('url')
  html = link_to('Click here', url)  # vuln-code-snippet vuln-line ruby_xss_link_to_href
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_link_to_href
