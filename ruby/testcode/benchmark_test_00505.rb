require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_custom_formatter(req)
  LOGGER.formatter = proc { |s, t, p, m| "#{t} #{req.param('context')}: #{m}\n" }
  LOGGER.info("processed")
  BenchmarkResponse.json({ ok: true })
end
