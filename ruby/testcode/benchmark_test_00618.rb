require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_elk_tag_inject(req)
  LOGGER.info("action=#{req.param('cmd')} status=ok user=#{req.param('user')}")
  BenchmarkResponse.json({ ok: true })
end
