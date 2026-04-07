require_relative 'shared'

def reflect_const_get(req)
  class_name = req.param('type')
  obj = Object.const_get(class_name).new
  BenchmarkResponse.ok(obj.to_s)
end
