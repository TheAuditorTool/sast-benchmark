require_relative 'shared'

def singleton_dispatch(req)
  name = req.param('name').to_sym
  obj  = Object.new
  obj.define_singleton_method(name) { 'dangerous' }
  result = obj.send(name)
  BenchmarkResponse.json({ result: result.to_s })
end
