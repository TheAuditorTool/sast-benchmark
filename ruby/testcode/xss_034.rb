require_relative 'shared'

# vuln-code-snippet start ruby_xss_content_tag_escaped
def render_safe_message_box(req)
  msg = req.param('msg')
  html = content_tag(:div, h(msg))  # vuln-code-snippet safe-line ruby_xss_content_tag_escaped
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_content_tag_escaped
