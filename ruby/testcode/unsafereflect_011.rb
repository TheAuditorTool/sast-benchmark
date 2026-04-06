require_relative 'shared'

# vuln-code-snippet start ruby_reflect_class_traverse
def instantiate_class(req)
  class_name = req.param('class')
  parts = class_name.split('::')
  klass = parts.reduce(Object) { |mod, name| mod.const_get(name) } # vuln-code-snippet vuln-line ruby_reflect_class_traverse
  BenchmarkResponse.ok(klass.new.to_s)
end
# vuln-code-snippet end ruby_reflect_class_traverse
