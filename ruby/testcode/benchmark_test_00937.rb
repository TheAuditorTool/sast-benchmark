require_relative 'shared'
require 'logger'

def log_semantic(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info({ event: :login, user_id: user_id }.inspect)
  BenchmarkResponse.ok('ok')
end
