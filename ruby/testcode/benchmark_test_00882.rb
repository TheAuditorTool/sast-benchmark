require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

def log_hash_only_params(req)
  LOGGER.info({ timestamp: Time.now.to_i, path: '/api/data', method: 'GET' }.to_json)
  BenchmarkResponse.json({ ok: true })
end
