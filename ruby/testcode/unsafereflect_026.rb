require_relative 'shared'

# vuln-code-snippet start ruby_reflect_method_dispatch2
def handler(req)
  obj = Object.new
  result = obj.method(req.param('action').to_sym).call(req.param('arg')) # vuln-code-snippet vuln-line ruby_reflect_method_dispatch2
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_method_dispatch2
