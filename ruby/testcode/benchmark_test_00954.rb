require_relative 'shared'

def reflect_new_from_string(req)
  handler_name = req.param('handler')
  data = req.param('data')
  klass = Object.const_get(handler_name)
  result = klass.new(data)
  BenchmarkResponse.ok(result.to_s)
end
