require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_objectspace_send
def objectspace_dispatch(req)
  method_sym = req.param('method').to_sym
  obj = ObjectSpace.each_object(String).first
  result = obj.send(method_sym) # vuln-code-snippet vuln-line ruby_dynmethod_objectspace_send
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_objectspace_send
