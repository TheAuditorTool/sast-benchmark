require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_gsub_control_chars(req)
  sanitized = req.param('val').gsub(/[[:cntrl:]]/, '?')
  LOGGER.info(sanitized)
  BenchmarkResponse.json({ ok: true })
end
