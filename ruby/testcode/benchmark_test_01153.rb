require_relative 'shared'

def handler(req)
  klass = Object.const_get("#{req.param('ns')}::#{req.param('klass')}")
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
