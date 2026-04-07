require_relative 'shared'

def handler(req)
  require_relative "../modules/#{req.param('mod')}"
  BenchmarkResponse.json({ ok: true })
end
