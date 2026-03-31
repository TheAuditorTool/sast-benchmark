require_relative 'shared'

# vuln-code-snippet start ruby_reflect_new_from_string
def reflect_new_from_string(req)
  handler_name = req.param('handler')
  data = req.param('data')
  klass = Object.const_get(handler_name) # vuln-code-snippet vuln-line ruby_reflect_new_from_string
  result = klass.new(data)
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_reflect_new_from_string
