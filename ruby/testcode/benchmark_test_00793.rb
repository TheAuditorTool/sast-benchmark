require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

def log_structured_hash(req)
  LOGGER.info({ user_id: 42, action: 'login', ip: '127.0.0.1' }.to_json)
  BenchmarkResponse.json({ ok: true })
end
