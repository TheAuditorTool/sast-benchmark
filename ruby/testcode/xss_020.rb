require_relative 'shared'

def erb_escape(value)
  escape_html(value.to_s)
end

# vuln-code-snippet start ruby_xss_erb_default_escape
def xss_erb_default_escape(req)
  title = req.param('title')
  body_text = req.param('body')
  safe_title = erb_escape(title) # vuln-code-snippet safe-line ruby_xss_erb_default_escape
  safe_body = erb_escape(body_text)
  html = "<main><h1>#{safe_title}</h1><article>#{safe_body}</article></main>"
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_erb_default_escape
