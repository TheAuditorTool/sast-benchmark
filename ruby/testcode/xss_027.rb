require_relative 'shared'

# vuln-code-snippet start ruby_xss_slim_unescaped
def render_bio_slim(req)
  bio = req.param('bio')
  # Slim == operator outputs unescaped HTML
  html = slim_render("== bio", locals: { bio: bio })  # vuln-code-snippet vuln-line ruby_xss_slim_unescaped
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_slim_unescaped
