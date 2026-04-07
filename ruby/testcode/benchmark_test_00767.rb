require_relative 'shared'

def render_safe_message_box(req)
  msg = req.param('msg')
  html = content_tag(:div, h(msg))
  BenchmarkResponse.html(html)
end
