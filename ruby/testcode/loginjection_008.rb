require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_gsub_newline
def log_gsub_safe(req)
  username = req.param('username')
  safe_name = username.gsub(/[\r\n]/, '') # vuln-code-snippet safe-line ruby_loginj_gsub_newline
  logger = Logger.new($stdout)
  logger.info("User #{safe_name} logged in")
  BenchmarkResponse.ok('logged in')
end
# vuln-code-snippet end ruby_loginj_gsub_newline
