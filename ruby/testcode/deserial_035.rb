require_relative 'shared'

# vuln-code-snippet start ruby_deser_psych_safe_empty_permitted
def psych_safe_empty_permitted_handler(req)
  data = req.post('data')
  obj = Psych.safe_load(data, permitted_classes: [])  # vuln-code-snippet safe-line ruby_deser_psych_safe_empty_permitted
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_psych_safe_empty_permitted
