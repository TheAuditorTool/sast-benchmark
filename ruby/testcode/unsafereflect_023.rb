require_relative 'shared'

# vuln-code-snippet start ruby_reflect_class_traverse2
def handler(req)
  klass = Module.const_get(req.param('mod')).const_get(req.param('klass')) # vuln-code-snippet vuln-line ruby_reflect_class_traverse2
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
# vuln-code-snippet end ruby_reflect_class_traverse2
