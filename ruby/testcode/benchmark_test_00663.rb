require_relative 'shared'

CLASS_MAP = { 'admin' => String, 'user' => Integer }.freeze

def handler(req)
  klass = CLASS_MAP.fetch(req.param('role'))
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
