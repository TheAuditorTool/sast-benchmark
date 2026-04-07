require_relative 'shared'
require 'logger'

def log_format_string(req)
  username = req.param('username')
  logger = Logger.new($stdout)
  logger.info("User %s logged in" % username)
  BenchmarkResponse.ok('logged in')
end
