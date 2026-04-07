require_relative 'shared'

REGISTRY = { 'user' => String, 'post' => Integer }.freeze

# vuln-code-snippet start ruby_reflect_registry_map
def handler(req)
  klass = REGISTRY.fetch(req.param('type')) # vuln-code-snippet safe-line ruby_reflect_registry_map
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
# vuln-code-snippet end ruby_reflect_registry_map
