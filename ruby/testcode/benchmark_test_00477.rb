require_relative 'shared'

def redirect_javascript_redirect(req)
  js = "window.location = '#{req.param('url')}';"
  BenchmarkResponse.html("<script>#{js}</script>")
end
