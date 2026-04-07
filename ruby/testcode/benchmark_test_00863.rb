require_relative 'shared'

REGISTRY = { 'user' => String, 'post' => Integer }.freeze

def handler(req)
  klass = REGISTRY.fetch(req.param('type'))
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
