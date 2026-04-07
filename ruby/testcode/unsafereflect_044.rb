require_relative 'shared'

CLASS_MAP = { 'admin' => String, 'user' => Integer }.freeze

# vuln-code-snippet start ruby_reflect_frozen_class_map
def handler(req)
  klass = CLASS_MAP.fetch(req.param('role')) # vuln-code-snippet safe-line ruby_reflect_frozen_class_map
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
# vuln-code-snippet end ruby_reflect_frozen_class_map
