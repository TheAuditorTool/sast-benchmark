require_relative 'shared'

def handler(req)
  input = req.param('input')
  label = req.param('label', default: 'Value')
  row = sprintf("<tr><td>%s</td><td>%s</td></tr>", label, input)
  table = "<table><thead><tr><th>Label</th><th>Data</th></tr></thead><tbody>#{row}</tbody></table>"
  BenchmarkResponse.html(table)
end
