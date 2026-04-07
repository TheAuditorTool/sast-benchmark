require_relative 'shared'

# vuln-code-snippet start ruby_reflect_module_function_dispatch
def handler(req)
  result = Module.new.extend(Kernel).send(req.param('method').to_sym) # vuln-code-snippet vuln-line ruby_reflect_module_function_dispatch
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_module_function_dispatch
