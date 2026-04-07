require_relative 'shared'

def handler(req)
  open(req.param('file')) { |f| eval(f.read) }
  BenchmarkResponse.json({ ok: true })
end
