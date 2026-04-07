require_relative 'shared'

def create_handler(req)
  type = req.param('type')
  klass = Object.const_get(type) rescue nil
  handler = klass&.new || Object.new
  BenchmarkResponse.ok(handler.to_s)
end
