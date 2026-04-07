require_relative 'shared'

class PluginHost; end

def register_plugin(req)
  method_name = req.param('name')
  body = req.param('body')
  PluginHost.define_method(method_name.to_sym) { body }
  BenchmarkResponse.ok("registered #{method_name}")
end
