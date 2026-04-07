require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_broadcast_taint(req)
  [LOGGER, Logger.new($stderr)].each { |l| l.info("Event: #{req.param('event')}") }
  BenchmarkResponse.json({ ok: true })
end
