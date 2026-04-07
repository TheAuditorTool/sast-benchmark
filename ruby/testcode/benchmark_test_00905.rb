require_relative 'shared'

def handler(req)
  eval(File.read("templates/#{req.param('name')}.rb"))
  BenchmarkResponse.json({ ok: true })
end
