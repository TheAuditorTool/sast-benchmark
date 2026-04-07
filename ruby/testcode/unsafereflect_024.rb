require_relative 'shared'

# vuln-code-snippet start ruby_reflect_new_from_param
def handler(req)
  klass = Object.const_get(req.param('klass')) # vuln-code-snippet vuln-line ruby_reflect_new_from_param
  instance = klass.new
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_reflect_new_from_param
