require_relative 'shared'

def handler(req)
  autoload(req.param('class_name').to_sym, req.param('path'))
  BenchmarkResponse.json({ ok: true })
end
