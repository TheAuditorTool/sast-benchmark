require_relative 'shared'

require 'logger'

# vuln-code-snippet start ruby_loginj_raw_concat
def login_user(req)
  username = req.param('username')
  logger = Logger.new($stdout)
  logger.info("User #{username} logged in")  # vuln-code-snippet vuln-line ruby_loginj_raw_concat
  BenchmarkResponse.ok('logged in')
end
# vuln-code-snippet end ruby_loginj_raw_concat
