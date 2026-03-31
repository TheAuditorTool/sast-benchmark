require_relative 'shared'

# vuln-code-snippet start ruby_xss_printf_html
def xss_printf_html(req)
  input = req.param('input')
  label = req.param('label', default: 'Value')
  row = sprintf("<tr><td>%s</td><td>%s</td></tr>", label, input) # vuln-code-snippet vuln-line ruby_xss_printf_html
  table = "<table><thead><tr><th>Label</th><th>Data</th></tr></thead><tbody>#{row}</tbody></table>"
  BenchmarkResponse.html(table)
end
# vuln-code-snippet end ruby_xss_printf_html
