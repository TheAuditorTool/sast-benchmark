require_relative 'shared'

# vuln-code-snippet start ruby_xss_haml_escaped
def render_safe_comment_haml(req)
  comment = req.param('comment')
  # Haml = operator auto-escapes HTML — safe by default
  html = haml_render("= comment", locals: { comment: comment })  # vuln-code-snippet safe-line ruby_xss_haml_escaped
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_haml_escaped
