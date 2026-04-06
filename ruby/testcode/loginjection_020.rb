require_relative 'shared'
require 'logger'

module Rails
  def self.logger; Logger.new($stdout); end
end

# vuln-code-snippet start ruby_loginj_tagged_constant
def log_tagged_constant(req)
  user_id = req.param('user_id').to_i
  Rails.logger.info("[auth] User #{user_id} authenticated") # vuln-code-snippet safe-line ruby_loginj_tagged_constant
  BenchmarkResponse.ok('ok')
end
# vuln-code-snippet end ruby_loginj_tagged_constant
