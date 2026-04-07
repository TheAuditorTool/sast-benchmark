require_relative 'shared'

# vuln-code-snippet start ruby_xss_haml_unescaped
def render_comment_haml(req)
  comment = req.param('comment')
  # Haml != operator renders unescaped HTML
  html = haml_render("!= comment", locals: { comment: comment })  # vuln-code-snippet vuln-line ruby_xss_haml_unescaped
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_haml_unescaped
