require_relative 'shared'

require 'logger'

def login_user(req)
  username = req.param('username')
  logger = Logger.new($stdout)
  logger.info("User #{username} logged in")
  BenchmarkResponse.ok('logged in')
end
