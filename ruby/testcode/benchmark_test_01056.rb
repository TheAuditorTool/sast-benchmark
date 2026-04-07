require_relative 'shared'

def eval_basic(req)
  expr = req.param('expr')
  result = BasicObject.new.instance_eval(expr)
  BenchmarkResponse.ok(result.to_s)
end
