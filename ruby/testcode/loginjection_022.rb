require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_tagged_user
def log_tagged_user(req)
  LOGGER.tagged(req.param('role')) { LOGGER.info("Access granted") } # vuln-code-snippet vuln-line ruby_loginj_tagged_user
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_tagged_user
