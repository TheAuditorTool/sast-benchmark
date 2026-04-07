require_relative 'shared'

# vuln-code-snippet start ruby_xss_content_tag_raw
def render_message_box(req)
  msg = req.param('msg')
  html = content_tag(:div, msg.html_safe)  # vuln-code-snippet vuln-line ruby_xss_content_tag_raw
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_content_tag_raw
