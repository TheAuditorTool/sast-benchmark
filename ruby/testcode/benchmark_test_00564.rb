require_relative 'shared'

def erb_escape(value)
  escape_html(value.to_s)
end

def handler(req)
  title = req.param('title')
  body_text = req.param('body')
  safe_title = erb_escape(title)
  safe_body = erb_escape(body_text)
  html = "<main><h1>#{safe_title}</h1><article>#{safe_body}</article></main>"
  BenchmarkResponse.html(html)
end
