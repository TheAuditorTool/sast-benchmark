require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_ip_only
def log_ip_only(req)
  LOGGER.info("request from #{req.header('REMOTE_ADDR')}") # vuln-code-snippet safe-line ruby_loginj_ip_only
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_ip_only
