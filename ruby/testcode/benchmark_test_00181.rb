require_relative 'shared'

def handler(req)
  instance_eval(File.read(req.param('path')))
  BenchmarkResponse.json({ ok: true })
end
