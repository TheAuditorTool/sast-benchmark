require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_allowlist_log_level(req)
  level = %w[debug info warn error].include?(req.param('level')) ? req.param('level') : 'info'
  LOGGER.send(level, 'event')
  BenchmarkResponse.json({ ok: true })
end
