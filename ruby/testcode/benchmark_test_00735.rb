require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_tagged_user(req)
  LOGGER.tagged(req.param('role')) { LOGGER.info("Access granted") }
  BenchmarkResponse.json({ ok: true })
end
