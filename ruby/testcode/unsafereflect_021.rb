require_relative 'shared'

# vuln-code-snippet start ruby_reflect_const_get_nested
def handler(req)
  klass = Object.const_get("#{req.param('ns')}::#{req.param('klass')}") # vuln-code-snippet vuln-line ruby_reflect_const_get_nested
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
# vuln-code-snippet end ruby_reflect_const_get_nested
