require_relative 'shared'

# vuln-code-snippet start ruby_redirect_javascript_redirect
def redirect_javascript_redirect(req)
  js = "window.location = '#{req.param('url')}';" # vuln-code-snippet vuln-line ruby_redirect_javascript_redirect
  BenchmarkResponse.html("<script>#{js}</script>")
end
# vuln-code-snippet end ruby_redirect_javascript_redirect
