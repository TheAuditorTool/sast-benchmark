require_relative 'shared'

def render_widget(req)
  payload = req.param('x')
  html = tag(:div, data: { payload: payload }, class: 'widget')
  BenchmarkResponse.html(html)
end
