require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_integer_only_calc
def add_integers(req)
  a = Integer(req.param('a')) # vuln-code-snippet safe-line ruby_codeinj_integer_only_calc
  b = Integer(req.param('b'))
  result = a + b
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_codeinj_integer_only_calc
