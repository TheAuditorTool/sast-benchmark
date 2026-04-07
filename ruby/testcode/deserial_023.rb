require_relative 'shared'

# vuln-code-snippet start ruby_deser_json_create_id
def json_create_id_handler(req)
  data = req.post('json')
  obj = JSON.load(data)  # vuln-code-snippet vuln-line ruby_deser_json_create_id
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_json_create_id
