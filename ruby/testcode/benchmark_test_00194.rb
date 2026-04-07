require_relative 'shared'

def instantiate_class(req)
  class_name = req.param('class')
  parts = class_name.split('::')
  klass = parts.reduce(Object) { |mod, name| mod.const_get(name) }
  BenchmarkResponse.ok(klass.new.to_s)
end
