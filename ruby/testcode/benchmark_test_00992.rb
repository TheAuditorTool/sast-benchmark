require_relative 'shared'

def handler(req)
  url = req.param('url')
  link_text = escape_html(req.param('text', default: 'Click here'))
  anchor = "<a href=\"#{url}\">#{link_text}</a>"
  BenchmarkResponse.html("<nav>#{anchor}</nav>")
end
