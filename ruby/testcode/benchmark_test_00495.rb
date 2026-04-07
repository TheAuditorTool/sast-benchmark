require_relative 'shared'

def handler(req)
  safe = req.param('klass').gsub(/[^A-Za-z0-9:]/, '')
  klass = Object.const_get(safe)
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
