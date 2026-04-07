require_relative 'shared'

def sandbox_eval(req)
  expr = req.param('expr')
  return BenchmarkResponse.bad_request('only numbers') unless expr.match?(/\A[\d+\-*\/() .]+\z/)
  result = eval(expr)
  BenchmarkResponse.ok(result.to_s)
end
