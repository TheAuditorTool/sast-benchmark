require_relative 'shared'

# vuln-code-snippet start ruby_deser_oj_custom_safe
def oj_custom_safe_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :strict, symbol_keys: false, create_id: nil)  # vuln-code-snippet safe-line ruby_deser_oj_custom_safe
  BenchmarkResponse.json({ result: obj })
end
# vuln-code-snippet end ruby_deser_oj_custom_safe
