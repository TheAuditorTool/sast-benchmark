require_relative 'shared'

MOD_MAP = { 'comparable' => Comparable, 'enumerable' => Enumerable }.freeze

# vuln-code-snippet start ruby_reflect_ancestors_allowlist
def handler(req)
  mod = MOD_MAP.fetch(req.param('mod')) # vuln-code-snippet safe-line ruby_reflect_ancestors_allowlist
  BenchmarkResponse.json({ ok: true, name: mod.name })
end
# vuln-code-snippet end ruby_reflect_ancestors_allowlist
