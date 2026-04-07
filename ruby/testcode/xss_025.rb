require_relative 'shared'

# vuln-code-snippet start ruby_xss_javascript_tag
def render_user_greeting(req)
  name = req.param('name')
  html = javascript_tag("var greeting = 'Hello, #{name}!';")  # vuln-code-snippet vuln-line ruby_xss_javascript_tag
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_javascript_tag
