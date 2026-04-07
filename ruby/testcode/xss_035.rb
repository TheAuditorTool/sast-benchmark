require_relative 'shared'

# vuln-code-snippet start ruby_xss_link_to_safe_url
def render_nav_link(req)
  path = req.param('path')
  # only_path: true strips host/scheme — prevents javascript: URI attacks
  safe_url = url_for(only_path: true, path: path)
  html = link_to('Navigate', safe_url)  # vuln-code-snippet safe-line ruby_xss_link_to_safe_url
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_link_to_safe_url
