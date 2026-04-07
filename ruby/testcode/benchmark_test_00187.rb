require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_id_integer_only(req)
  LOGGER.info("user_id=#{Integer(req.param('id'))} action=login")
  BenchmarkResponse.json({ ok: true })
end
