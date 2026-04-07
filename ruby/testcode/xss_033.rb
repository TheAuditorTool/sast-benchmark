require_relative 'shared'

# vuln-code-snippet start ruby_xss_data_attr_raw
def render_widget(req)
  payload = req.param('x')
  # In old Rails, data attribute values are not escaped — XSS via data-* if rendered as HTML
  html = tag(:div, data: { payload: payload }, class: 'widget')  # vuln-code-snippet vuln-line ruby_xss_data_attr_raw
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_data_attr_raw
