require_relative 'shared'
require 'logger'

module Rails
  def self.logger; Logger.new($stdout); end
end

def log_tagged_constant(req)
  user_id = req.param('user_id').to_i
  Rails.logger.info("[auth] User #{user_id} authenticated")
  BenchmarkResponse.ok('ok')
end
