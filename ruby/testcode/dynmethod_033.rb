require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_module_nesting
def module_dispatch(req)
  method_name = req.param('method_name').to_sym
  result = Module.new.tap { |m| m.send(method_name) } # vuln-code-snippet vuln-line ruby_dynmethod_module_nesting
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_module_nesting
