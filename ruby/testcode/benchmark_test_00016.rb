require_relative 'shared'

def objectspace_dispatch(req)
  method_sym = req.param('method').to_sym
  obj = ObjectSpace.each_object(String).first
  result = obj.send(method_sym)
  BenchmarkResponse.json({ result: result.to_s })
end
