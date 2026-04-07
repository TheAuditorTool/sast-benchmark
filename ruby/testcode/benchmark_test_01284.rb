require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

def log_lograge_config(req)
  LOGGER.info({ path: req.header('REQUEST_URI'), status: 200 }.to_json)
  BenchmarkResponse.json({ ok: true })
end
