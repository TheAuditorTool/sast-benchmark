require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_define_constant
def register_greeting(req)
  DynamicWorker = Class.new unless defined?(DynamicWorker)
  DynamicWorker.define_method(:greet) { |name| "Hello, #{name}" } # vuln-code-snippet safe-line ruby_dynmethod_define_constant
  result = DynamicWorker.new.greet(req.param('name'))
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_dynmethod_define_constant
