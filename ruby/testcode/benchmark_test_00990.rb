require_relative 'shared'

def configure_object(req)
  expr = req.param('expr')
  obj = Object.new
  obj.instance_eval(expr)
  BenchmarkResponse.ok("configured")
end
