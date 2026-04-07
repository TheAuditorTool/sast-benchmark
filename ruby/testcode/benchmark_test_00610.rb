require_relative 'shared'
require 'logger'

def log_allowlist(req)
  username = req.param('username')
  safe_name = username.gsub(/[^a-zA-Z0-9_]/, '')
  logger = Logger.new($stdout)
  logger.info("User #{safe_name} logged in")
  BenchmarkResponse.ok('ok')
end
