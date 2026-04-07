require_relative 'shared'

# vuln-code-snippet start ruby_xss_style_inject
def render_colored_label(req)
  color = req.param('color')
  html = tag(:span, style: "color: #{color}", class: 'label')  # vuln-code-snippet vuln-line ruby_xss_style_inject
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_style_inject
