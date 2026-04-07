require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_const_send
def const_dispatch(req)
  klass  = Object.const_get(req.param('klass'))  # vuln-code-snippet vuln-line ruby_dynmethod_const_send
  result = klass.send(req.param('method'))
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_const_send
