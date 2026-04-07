require_relative 'shared'

def eval_expression(req)
  expression = req.param('expression')
  x = 42
  result = binding.eval(expression)
  BenchmarkResponse.ok(result.to_s)
end
