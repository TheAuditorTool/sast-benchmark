require_relative 'shared'

def handler(req)
  klass = Module.const_get(req.param('mod')).const_get(req.param('klass'))
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
