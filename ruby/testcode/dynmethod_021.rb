require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_method_call_taint
def dispatch_via_method(req)
  target = "hello"
  m = method(req.param('name').to_sym) # vuln-code-snippet vuln-line ruby_dynmethod_method_call_taint
  result = m.call(target)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_method_call_taint
