require_relative 'shared'

require 'logger'
require 'json'

def login_user(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info(JSON.generate({ event: 'login', user_id: user_id }))
  BenchmarkResponse.ok('logged in')
end
