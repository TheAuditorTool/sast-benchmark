require_relative 'shared'

def handler(req)
  require 'json'
  BenchmarkResponse.json({ ok: true })
end
