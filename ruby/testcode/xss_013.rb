require_relative 'shared'

# vuln-code-snippet start ruby_xss_attr_unquoted
def xss_attr_unquoted(req)
  cls = req.param('class')
  theme = req.param('theme', default: 'light')
  label = escape_html(req.param('label'))
  widget = "<div class=#{cls} data-theme=#{theme}>#{label}</div>" # vuln-code-snippet vuln-line ruby_xss_attr_unquoted
  BenchmarkResponse.html(widget)
end
# vuln-code-snippet end ruby_xss_attr_unquoted
