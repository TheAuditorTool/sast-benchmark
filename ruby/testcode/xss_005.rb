require_relative 'shared'

# vuln-code-snippet start ruby_xss_concat_html
def xss_concat_html(req)
  name = req.param('name')
  role = req.cookie('role')
  html = "<p>Welcome, " + name + "!</p>" # vuln-code-snippet vuln-line ruby_xss_concat_html
  html += "<span class=\"role\">" + role + "</span>"
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_concat_html
