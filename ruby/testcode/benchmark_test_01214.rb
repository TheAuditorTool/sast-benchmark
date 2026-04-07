require_relative 'shared'

def handler(req)
  env = 'production'
  load(File.join('config', "#{env}.rb"))
  BenchmarkResponse.json({ ok: true })
end
