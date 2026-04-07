require_relative 'shared'

def handler(req)
  autoload(req.param('const').to_sym, req.param('path'))
  BenchmarkResponse.json({ ok: true })
end
