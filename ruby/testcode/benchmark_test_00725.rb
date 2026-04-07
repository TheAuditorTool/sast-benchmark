require_relative 'shared'

def handler(req)
  cls = req.param('class', default: 'default')
  label = req.param('label')
  safe_cls = escape_html(cls)
  safe_label = escape_html(label)
  widget = "<div class=\"#{safe_cls}\">#{safe_label}</div>"
  BenchmarkResponse.html(widget)
end
