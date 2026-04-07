require_relative 'shared'

# vuln-code-snippet start ruby_deser_oj_compat
def oj_compat_deserialize_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :compat)  # vuln-code-snippet vuln-line ruby_deser_oj_compat
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_oj_compat
