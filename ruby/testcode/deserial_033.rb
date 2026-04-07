require_relative 'shared'

# vuln-code-snippet start ruby_deser_oj_strict_mode
def oj_strict_mode_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :strict)  # vuln-code-snippet safe-line ruby_deser_oj_strict_mode
  BenchmarkResponse.json({ result: obj })
end
# vuln-code-snippet end ruby_deser_oj_strict_mode
