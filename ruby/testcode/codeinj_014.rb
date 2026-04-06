require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_integer_calc
def safe_calculate(req)
  a = req.param('a').to_i
  b = req.param('b').to_i
  result = a + b # vuln-code-snippet safe-line ruby_codeinj_integer_calc
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_integer_calc
