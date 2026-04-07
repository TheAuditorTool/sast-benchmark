require_relative 'shared'

def evaluate_and_catch(req)
  expr = req.param('expr')
  result = catch(:result) { throw :result, eval(expr) }
  BenchmarkResponse.json({ result: result.to_s })
end
