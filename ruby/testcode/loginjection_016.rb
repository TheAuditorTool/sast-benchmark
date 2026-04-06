require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_integer_only
def log_integer_safe(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info("Login user_id=#{user_id}") # vuln-code-snippet safe-line ruby_loginj_integer_only
  BenchmarkResponse.ok('ok')
end
# vuln-code-snippet end ruby_loginj_integer_only
