require_relative 'shared'

# vuln-code-snippet start ruby_reflect_instance_method_bind
def handler(req)
  result = Object.instance_method(req.param('method').to_sym).bind(Object.new).call # vuln-code-snippet vuln-line ruby_reflect_instance_method_bind
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_instance_method_bind
