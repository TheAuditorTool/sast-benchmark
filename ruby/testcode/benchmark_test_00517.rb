require_relative 'shared'

MOD_MAP = { 'comparable' => Comparable, 'enumerable' => Enumerable }.freeze

def handler(req)
  mod = MOD_MAP.fetch(req.param('mod'))
  BenchmarkResponse.json({ ok: true, name: mod.name })
end
