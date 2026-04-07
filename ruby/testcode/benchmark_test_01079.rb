require_relative 'shared'

def handler(req)
  load(File.join('plugins', req.param('plugin') + '.rb'))
  BenchmarkResponse.json({ ok: true })
end
