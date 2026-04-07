require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_no_user_in_log(req)
  LOGGER.info("request processed at #{Time.now}")
  BenchmarkResponse.json({ ok: true })
end
