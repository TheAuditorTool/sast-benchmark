require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_then_eval
def transform_value(req)
  transform = req.param('transform')
  value = 42
  result = value.then { eval(transform) } # vuln-code-snippet vuln-line ruby_codeinj_then_eval
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_then_eval
