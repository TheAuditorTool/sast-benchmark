require_relative 'shared'
require 'logger'

def log_gsub_safe(req)
  username = req.param('username')
  safe_name = username.gsub(/[\r\n]/, '')
  logger = Logger.new($stdout)
  logger.info("User #{safe_name} logged in")
  BenchmarkResponse.ok('logged in')
end
