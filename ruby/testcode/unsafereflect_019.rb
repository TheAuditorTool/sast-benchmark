require_relative 'shared'

# vuln-code-snippet start ruby_reflect_safe_const_nil
def create_handler(req)
  type = req.param('type')
  klass = Object.const_get(type) rescue nil
  handler = klass&.new || Object.new # vuln-code-snippet vuln-line ruby_reflect_safe_const_nil
  BenchmarkResponse.ok(handler.to_s)
end
# vuln-code-snippet end ruby_reflect_safe_const_nil
