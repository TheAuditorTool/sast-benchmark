require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_binding_local
def binding_dispatch(req)
  x = 42
  var_name = req.param('var_name').to_sym
  result = binding.local_variable_get(var_name) # vuln-code-snippet vuln-line ruby_dynmethod_binding_local
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_binding_local
