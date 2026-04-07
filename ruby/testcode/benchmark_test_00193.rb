require_relative 'shared'

def handler(req)
  result = binding.eval("#{req.param('klass')}.new.#{req.param('method')}")
  BenchmarkResponse.json({ result: result.to_s })
end
