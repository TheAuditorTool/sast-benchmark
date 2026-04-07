require_relative 'shared'

# vuln-code-snippet start ruby_deser_json_parse_strict
def json_parse_strict_handler(req)
  data = req.post('data')
  obj = JSON.parse(data, symbolize_names: false)  # vuln-code-snippet safe-line ruby_deser_json_parse_strict
  BenchmarkResponse.json({ result: obj })
end
# vuln-code-snippet end ruby_deser_json_parse_strict
