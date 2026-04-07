require_relative 'shared'

# vuln-code-snippet start ruby_deser_psych_load
def psych_deserialize_handler(req)
  data = req.post('data')
  obj = Psych.load(data)  # vuln-code-snippet vuln-line ruby_deser_psych_load
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_psych_load
