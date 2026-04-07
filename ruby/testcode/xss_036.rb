require_relative 'shared'

# vuln-code-snippet start ruby_xss_javascript_tag_j
def render_user_init_script(req)
  name = req.param('name')
  # j() (escape_javascript) escapes quotes, backslashes, newlines — safe in JS context
  html = javascript_tag("var username = '#{j(name)}';")  # vuln-code-snippet safe-line ruby_xss_javascript_tag_j
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_javascript_tag_j
