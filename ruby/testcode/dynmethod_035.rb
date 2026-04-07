require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_define_singleton
def singleton_dispatch(req)
  name = req.param('name').to_sym
  obj  = Object.new
  obj.define_singleton_method(name) { 'dangerous' } # vuln-code-snippet vuln-line ruby_dynmethod_define_singleton
  result = obj.send(name)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_define_singleton
