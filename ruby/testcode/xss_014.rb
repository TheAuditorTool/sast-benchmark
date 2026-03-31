require_relative 'shared'

# vuln-code-snippet start ruby_xss_attr_escaped
def xss_attr_escaped(req)
  cls = req.param('class', default: 'default')
  label = req.param('label')
  safe_cls = escape_html(cls) # vuln-code-snippet safe-line ruby_xss_attr_escaped
  safe_label = escape_html(label)
  widget = "<div class=\"#{safe_cls}\">#{safe_label}</div>"
  BenchmarkResponse.html(widget)
end
# vuln-code-snippet end ruby_xss_attr_escaped
