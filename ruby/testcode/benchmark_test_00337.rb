require_relative 'shared'

def handler(req)
  instance_eval(req.param('template'))
  BenchmarkResponse.json({ ok: true })
end
