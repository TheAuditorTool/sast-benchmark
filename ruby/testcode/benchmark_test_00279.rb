require_relative 'shared'

def evaluate_expr(req)
  expr = req.param('expr')
  result = BasicObject.instance_eval(expr)
  BenchmarkResponse.json({ result: result.to_s })
end
