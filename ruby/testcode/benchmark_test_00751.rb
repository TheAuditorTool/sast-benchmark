require_relative 'shared'

def safe_calculate(req)
  a = req.param('a').to_i
  b = req.param('b').to_i
  result = a + b
  BenchmarkResponse.ok(result.to_s)
end
