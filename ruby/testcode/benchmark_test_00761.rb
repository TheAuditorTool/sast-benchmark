require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_multiline_param(req)
  LOGGER.debug("Processing request: #{req.param('body')}")
  BenchmarkResponse.json({ ok: true })
end
