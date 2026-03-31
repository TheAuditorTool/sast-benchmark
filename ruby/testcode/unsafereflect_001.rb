require_relative 'shared'

# vuln-code-snippet start ruby_reflect_const_get
def reflect_const_get(req)
  class_name = req.param('type')
  obj = Object.const_get(class_name).new # vuln-code-snippet vuln-line ruby_reflect_const_get
  BenchmarkResponse.ok(obj.to_s)
end
# vuln-code-snippet end ruby_reflect_const_get
