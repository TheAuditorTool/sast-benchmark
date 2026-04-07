require_relative 'shared'

# vuln-code-snippet start ruby_xss_strip_all
def render_plain_comment(req)
  comment = req.param('comment')
  plain_text = strip_tags(comment)  # vuln-code-snippet safe-line ruby_xss_strip_all
  BenchmarkResponse.html("<p>#{h(plain_text)}</p>")
end
# vuln-code-snippet end ruby_xss_strip_all
