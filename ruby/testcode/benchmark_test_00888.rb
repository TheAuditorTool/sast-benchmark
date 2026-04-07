require_relative 'shared'

def module_dispatch(req)
  method_name = req.param('method_name').to_sym
  result = Module.new.tap { |m| m.send(method_name) }
  BenchmarkResponse.json({ result: result.to_s })
end
