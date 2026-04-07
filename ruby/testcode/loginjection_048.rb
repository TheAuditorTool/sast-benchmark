require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_truncate_sanitize
def log_truncate_sanitize(req)
  sanitized = req.param('val').gsub(/[^\w\s@.\-]/, '')[0..100]
  LOGGER.info(sanitized) # vuln-code-snippet safe-line ruby_loginj_truncate_sanitize
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_truncate_sanitize
