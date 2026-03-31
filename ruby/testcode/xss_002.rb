require_relative 'shared'

# vuln-code-snippet start ruby_xss_escape_html
def xss_escape_html(req)
  username = req.param('username')
  message = req.param('message')
  safe_name = escape_html(username) # vuln-code-snippet safe-line ruby_xss_escape_html
  safe_msg = escape_html(message)
  body = "<div><strong>#{safe_name}</strong>: #{safe_msg}</div>"
  BenchmarkResponse.html(body)
end
# vuln-code-snippet end ruby_xss_escape_html
