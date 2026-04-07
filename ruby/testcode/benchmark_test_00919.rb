require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_ip_only(req)
  LOGGER.info("request from #{req.header('REMOTE_ADDR')}")
  BenchmarkResponse.json({ ok: true })
end
