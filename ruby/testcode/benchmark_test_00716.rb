require_relative 'shared'

def call_method_object(req)
  method_name = req.param('method')
  arg = req.param('arg')
  result = method(method_name.to_sym).call(arg)
  BenchmarkResponse.ok(result.to_s)
end
