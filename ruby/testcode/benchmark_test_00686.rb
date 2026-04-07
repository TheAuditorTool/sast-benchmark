require_relative 'shared'

def raw(s)
  s
end

def handler(req)
  title = req.param('title')
  description = req.post('description')
  content = raw(description)
  page = "<article><h1>#{escape_html(title)}</h1><section>#{content}</section></article>"
  BenchmarkResponse.html(page)
end
