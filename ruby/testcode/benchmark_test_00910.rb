require_relative 'shared'

def render_colored_label(req)
  color = req.param('color')
  html = tag(:span, style: "color: #{color}", class: 'label')
  BenchmarkResponse.html(html)
end
