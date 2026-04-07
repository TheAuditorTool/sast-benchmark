require_relative 'shared'

def handler(req)
  klass = Object.const_get(req.param('klass'))
  instance = klass.new
  BenchmarkResponse.json({ ok: true })
end
