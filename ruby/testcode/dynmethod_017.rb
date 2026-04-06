require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_method_object
def call_method_object(req)
  method_name = req.param('method')
  arg = req.param('arg')
  result = method(method_name.to_sym).call(arg) # vuln-code-snippet vuln-line ruby_dynmethod_method_object
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_dynmethod_method_object
