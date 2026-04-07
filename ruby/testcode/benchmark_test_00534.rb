require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

def log_json_encode_value(req)
  LOGGER.info(JSON.generate({ event: req.param('action'), user: 'system' }))
  BenchmarkResponse.json({ ok: true })
end
