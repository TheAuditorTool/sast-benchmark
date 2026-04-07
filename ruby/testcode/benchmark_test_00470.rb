require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_semantic_level(req)
  LOGGER.warn("user=#{req.param('class_name')} accessed admin area")
  BenchmarkResponse.json({ ok: true })
end
