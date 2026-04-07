require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

def log_json_key_inject(req)
  LOGGER.info({ req.param('key') => 'value' }.to_json)
  BenchmarkResponse.json({ ok: true })
end
