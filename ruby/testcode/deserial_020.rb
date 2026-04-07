require_relative 'shared'

# vuln-code-snippet start ruby_deser_oj_rails
def oj_rails_deserialize_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :rails)  # vuln-code-snippet vuln-line ruby_deser_oj_rails
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_oj_rails
