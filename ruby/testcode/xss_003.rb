require_relative 'shared'

def raw(s)
  s
end

# vuln-code-snippet start ruby_xss_raw_helper
def xss_raw_helper(req)
  title = req.param('title')
  description = req.post('description')
  content = raw(description) # vuln-code-snippet vuln-line ruby_xss_raw_helper
  page = "<article><h1>#{escape_html(title)}</h1><section>#{content}</section></article>"
  BenchmarkResponse.html(page)
end
# vuln-code-snippet end ruby_xss_raw_helper
