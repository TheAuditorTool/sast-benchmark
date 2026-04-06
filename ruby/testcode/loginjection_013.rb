require_relative 'shared'
require 'logger'

module Rails
  def self.logger; Logger.new($stdout); end
end

# vuln-code-snippet start ruby_loginj_rails_tagged
def log_tagged_user(req)
  tag = req.param('tag')
  Rails.logger.info("[#{tag}] Request processed") # vuln-code-snippet vuln-line ruby_loginj_rails_tagged
  BenchmarkResponse.ok('ok')
end
# vuln-code-snippet end ruby_loginj_rails_tagged
