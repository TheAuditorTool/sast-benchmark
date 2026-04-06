require_relative 'shared'

class PluginHost; end

# vuln-code-snippet start ruby_dynmethod_define_from_hash
def register_plugin(req)
  method_name = req.param('name')
  body = req.param('body')
  PluginHost.define_method(method_name.to_sym) { body } # vuln-code-snippet vuln-line ruby_dynmethod_define_from_hash
  BenchmarkResponse.ok("registered #{method_name}")
end
# vuln-code-snippet end ruby_dynmethod_define_from_hash
