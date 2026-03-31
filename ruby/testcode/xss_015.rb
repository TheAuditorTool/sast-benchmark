require_relative 'shared'

# vuln-code-snippet start ruby_xss_href_javascript
def xss_href_javascript(req)
  url = req.param('url')
  link_text = escape_html(req.param('text', default: 'Click here'))
  anchor = "<a href=\"#{url}\">#{link_text}</a>" # vuln-code-snippet vuln-line ruby_xss_href_javascript
  BenchmarkResponse.html("<nav>#{anchor}</nav>")
end
# vuln-code-snippet end ruby_xss_href_javascript
