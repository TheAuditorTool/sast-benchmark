require_relative 'shared'

# vuln-code-snippet start ruby_deser_oj_wab
def oj_wab_deserialize_handler(req)
  data = req.post('data')
  obj = Oj.wab_load(data)  # vuln-code-snippet vuln-line ruby_deser_oj_wab
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_oj_wab
