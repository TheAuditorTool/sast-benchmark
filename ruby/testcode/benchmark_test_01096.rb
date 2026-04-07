require_relative 'shared'

def eval_in_thread(req)
  code = req.param('code')
  result = nil
  t = Thread.new { result = eval(code) }
  t.join(5)
  BenchmarkResponse.ok(result.to_s)
end
