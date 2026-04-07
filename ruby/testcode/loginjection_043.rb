require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_allowlist_log_level
def log_allowlist_log_level(req)
  level = %w[debug info warn error].include?(req.param('level')) ? req.param('level') : 'info'
  LOGGER.send(level, 'event') # vuln-code-snippet safe-line ruby_loginj_allowlist_log_level
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_allowlist_log_level
