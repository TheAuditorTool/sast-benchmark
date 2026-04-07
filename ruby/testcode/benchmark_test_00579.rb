require_relative 'shared'
require 'logger'

module Rails
  def self.logger; Logger.new($stdout); end
end

def log_tagged_user(req)
  tag = req.param('tag')
  Rails.logger.info("[#{tag}] Request processed")
  BenchmarkResponse.ok('ok')
end
