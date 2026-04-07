require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_warn_concat(req)
  LOGGER.warn("Failed login for: " + req.param('user'))
  BenchmarkResponse.json({ ok: false })
end
