require_relative 'shared'

FIXED_NAME_041 = 'world'.freeze

# vuln-code-snippet start ruby_dynmethod_define_constant_body
def define_constant_body(req)
  klass = Class.new
  klass.define_method(:greet) { "hello #{FIXED_NAME_041}" } # vuln-code-snippet safe-line ruby_dynmethod_define_constant_body
  result = klass.new.greet
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_define_constant_body
