require_relative 'shared'

def handler(req)
  Gem.find_files('plugins/*.rb').each { |f| require f }
  BenchmarkResponse.json({ ok: true })
end
