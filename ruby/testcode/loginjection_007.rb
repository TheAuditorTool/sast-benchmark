require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_format_string
def log_format_string(req)
  username = req.param('username')
  logger = Logger.new($stdout)
  logger.info("User %s logged in" % username) # vuln-code-snippet vuln-line ruby_loginj_format_string
  BenchmarkResponse.ok('logged in')
end
# vuln-code-snippet end ruby_loginj_format_string
