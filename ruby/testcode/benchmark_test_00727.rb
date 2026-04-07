require_relative 'shared'

def eval_body(req)
  code = req.body_str
  result = Kernel.eval(code)
  BenchmarkResponse.ok(result.to_s)
end
