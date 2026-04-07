require_relative 'shared'
require 'json'
require 'logger'

def log_json_structured(req)
  username = req.param('username')
  action = req.param('action')
  logger = Logger.new($stdout)
  logger.info(JSON.generate({ event: 'action', user: username, action: action }))
  BenchmarkResponse.ok('ok')
end
