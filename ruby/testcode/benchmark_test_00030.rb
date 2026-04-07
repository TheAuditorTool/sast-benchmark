require_relative 'shared'

def handler(req)
  load("views/#{req.param('view')}.rb")
  BenchmarkResponse.json({ ok: true })
end
