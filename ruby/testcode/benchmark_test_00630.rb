require_relative 'shared'

def handler(req)
  Kernel.load("/app/extensions/#{req.param('ext')}.rb")
  BenchmarkResponse.json({ ok: true })
end
