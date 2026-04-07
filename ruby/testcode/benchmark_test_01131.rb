require_relative 'shared'

def handler(req)
  cls = req.param('class')
  theme = req.param('theme', default: 'light')
  label = escape_html(req.param('label'))
  widget = "<div class=#{cls} data-theme=#{theme}>#{label}</div>"
  BenchmarkResponse.html(widget)
end
