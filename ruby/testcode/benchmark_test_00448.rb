require_relative 'shared'

def handler(req)
  autoload(:Plugin, "http://#{req.param('host')}/plugin.rb")
  BenchmarkResponse.json({ ok: true })
end
