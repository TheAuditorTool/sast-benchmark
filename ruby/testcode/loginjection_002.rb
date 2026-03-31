require_relative 'shared'

require 'logger'

# vuln-code-snippet start ruby_loginj_sanitized
def login_user_sanitized(req)
  username = req.param('username')
  logger = Logger.new($stdout)
  safe_name = username.gsub(/[\r\n\t]/, '_')
  logger.info("User #{safe_name} logged in")  # vuln-code-snippet safe-line ruby_loginj_sanitized
  BenchmarkResponse.ok('logged in')
end
# vuln-code-snippet end ruby_loginj_sanitized
