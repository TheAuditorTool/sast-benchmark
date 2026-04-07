require_relative 'shared'

def register_greeting(req)
  DynamicWorker = Class.new unless defined?(DynamicWorker)
  DynamicWorker.define_method(:greet) { |name| "Hello, #{name}" }
  result = DynamicWorker.new.greet(req.param('name'))
  BenchmarkResponse.ok(result)
end
