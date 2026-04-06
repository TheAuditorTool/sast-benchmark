require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_struct_dynamic
def create_struct(req)
  fields = req.param('fields').split(',').map(&:to_sym)
  klass = Struct.new(*fields) # vuln-code-snippet vuln-line ruby_dynmethod_struct_dynamic
  instance = klass.new(*fields.map { |f| req.param(f.to_s) })
  BenchmarkResponse.ok(instance.to_a.join(', '))
end
# vuln-code-snippet end ruby_dynmethod_struct_dynamic
