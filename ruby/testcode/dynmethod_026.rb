require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_then_dispatch
def then_dispatch(req)
  transform = req.param('transform')
  result = 42.then { |v| v.send(transform) } # vuln-code-snippet vuln-line ruby_dynmethod_then_dispatch
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_then_dispatch
