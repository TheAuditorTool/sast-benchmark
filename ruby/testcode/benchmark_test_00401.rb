require_relative 'shared'

def handler(req)
  Kernel.send(req.param('method').to_sym, req.param('cmd'))
  BenchmarkResponse.json({ ok: true })
end
