require_relative 'shared'
require 'logger'

def log_integer_safe(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info("Login user_id=#{user_id}")
  BenchmarkResponse.ok('ok')
end
