require_relative 'shared'

def render_message_box(req)
  msg = req.param('msg')
  html = content_tag(:div, msg.html_safe)
  BenchmarkResponse.html(html)
end
