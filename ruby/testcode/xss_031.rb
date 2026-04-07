require_relative 'shared'

# vuln-code-snippet start ruby_xss_tilt_render
def render_custom_template(req)
  template_str = req.param('template')
  # Tilt renders arbitrary ERB — user-controlled template = code execution + XSS
  output = Tilt['erb'].new { template_str }.render  # vuln-code-snippet vuln-line ruby_xss_tilt_render
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_xss_tilt_render
