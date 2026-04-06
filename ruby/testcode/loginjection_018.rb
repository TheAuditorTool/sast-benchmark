require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_allowlist_chars
def log_allowlist(req)
  username = req.param('username')
  safe_name = username.gsub(/[^a-zA-Z0-9_]/, '') # vuln-code-snippet safe-line ruby_loginj_allowlist_chars
  logger = Logger.new($stdout)
  logger.info("User #{safe_name} logged in")
  BenchmarkResponse.ok('ok')
end
# vuln-code-snippet end ruby_loginj_allowlist_chars
