require_relative 'shared'

def dispatch_method(req)
  method_name = req.param('method_name')
  ctx = binding
  result = ctx.eval("self.#{method_name}")
  BenchmarkResponse.json({ result: result.to_s })
end
