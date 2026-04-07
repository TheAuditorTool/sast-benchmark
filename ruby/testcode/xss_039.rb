require_relative 'shared'

# vuln-code-snippet start ruby_xss_sanitize_allowlist
def render_rich_text(req)
  body = req.param('body')
  # sanitize with strict allowlist — only safe tags/attrs pass through
  clean = sanitize(body, tags: %w[p b i em strong], attributes: %w[class])  # vuln-code-snippet safe-line ruby_xss_sanitize_allowlist
  BenchmarkResponse.html("<article>#{clean}</article>")
end
# vuln-code-snippet end ruby_xss_sanitize_allowlist
