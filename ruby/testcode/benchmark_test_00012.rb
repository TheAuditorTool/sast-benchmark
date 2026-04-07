require_relative 'shared'

require 'logger'

def login_user(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info("Login event for user_id=#{user_id}")
  BenchmarkResponse.ok('logged in')
end
