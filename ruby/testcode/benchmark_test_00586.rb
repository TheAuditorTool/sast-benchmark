require_relative 'shared'

def create_struct(req)
  fields = req.param('fields').split(',').map(&:to_sym)
  klass = Struct.new(*fields)
  instance = klass.new(*fields.map { |f| req.param(f.to_s) })
  BenchmarkResponse.ok(instance.to_a.join(', '))
end
