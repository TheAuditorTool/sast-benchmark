require_relative 'shared'

# vuln-code-snippet start ruby_xss_html_safe
def xss_html_safe(req)
  name = req.param('name')
  greeting = req.param('greeting', default: 'Hello')
  body = "<div class=\"greeting\">#{greeting}, #{name.html_safe}!</div>" # vuln-code-snippet vuln-line ruby_xss_html_safe
  BenchmarkResponse.html(body)
end
# vuln-code-snippet end ruby_xss_html_safe
