require_relative 'shared'

FIXED_NAME_041 = 'world'.freeze

def define_constant_body(req)
  klass = Class.new
  klass.define_method(:greet) { "hello #{FIXED_NAME_041}" }
  result = klass.new.greet
  BenchmarkResponse.json({ result: result })
end
