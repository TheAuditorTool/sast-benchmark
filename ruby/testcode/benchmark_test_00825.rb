require_relative 'shared'

def handler(req)
  require 'json'
  require 'openssl'
  BenchmarkResponse.json({ ok: true })
end
