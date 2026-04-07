require_relative 'shared'

# vuln-code-snippet start ruby_xss_text_node
def render_username_display(req)
  name = req.param('name')
  safe_name = ERB::Util.html_escape(name)  # vuln-code-snippet safe-line ruby_xss_text_node
  BenchmarkResponse.html("<span class='username'>#{safe_name}</span>")
end
# vuln-code-snippet end ruby_xss_text_node
