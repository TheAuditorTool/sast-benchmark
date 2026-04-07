require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_warn_concat
def log_warn_concat(req)
  LOGGER.warn("Failed login for: " + req.param('user')) # vuln-code-snippet vuln-line ruby_loginj_warn_concat
  BenchmarkResponse.json({ ok: false })
end
# vuln-code-snippet end ruby_loginj_warn_concat
