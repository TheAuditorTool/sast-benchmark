require_relative 'shared'

require 'logger'

def login_user_sanitized(req)
  username = req.param('username')
  logger = Logger.new($stdout)
  safe_name = username.gsub(/[\r\n\t]/, '_')
  logger.info("User #{safe_name} logged in")
  BenchmarkResponse.ok('logged in')
end
