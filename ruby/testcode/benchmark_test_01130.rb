require_relative 'shared'

def dispatch_via_method(req)
  target = "hello"
  m = method(req.param('name').to_sym)
  result = m.call(target)
  BenchmarkResponse.json({ result: result.to_s })
end
