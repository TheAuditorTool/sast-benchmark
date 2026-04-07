require_relative 'shared'

def handler(req)
  load(File.join('config', req.param('env') + '.rb'))
  BenchmarkResponse.json({ ok: true })
end
