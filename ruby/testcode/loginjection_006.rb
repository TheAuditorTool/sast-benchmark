require_relative 'shared'

require 'logger'

# vuln-code-snippet start ruby_loginj_id_only
def login_user(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info("Login event for user_id=#{user_id}")  # vuln-code-snippet safe-line ruby_loginj_id_only
  BenchmarkResponse.ok('logged in')
end
# vuln-code-snippet end ruby_loginj_id_only
