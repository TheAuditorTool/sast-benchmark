require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_safe_calculate
def calculate(req)
  a = Integer(req.param('a')) # vuln-code-snippet safe-line ruby_codeinj_safe_calculate
  b = Integer(req.param('b'))
  op = req.param('op')
  result = case op
           when 'add' then a + b
           when 'sub' then a - b
           when 'mul' then a * b
           else 0
           end
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_safe_calculate
