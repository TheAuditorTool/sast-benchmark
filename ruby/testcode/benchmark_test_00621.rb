require_relative 'shared'
require 'json'
require 'logger'

def log_lograge_style(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  payload = { event: 'login', user_id: user_id, status: 'success' }
  logger.info(JSON.generate(payload))
  BenchmarkResponse.ok('logged in')
end
