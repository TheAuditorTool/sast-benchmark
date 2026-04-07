require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_truncate_sanitize(req)
  sanitized = req.param('val').gsub(/[^\w\s@.\-]/, '')[0..100]
  LOGGER.info(sanitized)
  BenchmarkResponse.json({ ok: true })
end
