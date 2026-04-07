require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_id_integer_only
def log_id_integer_only(req)
  LOGGER.info("user_id=#{Integer(req.param('id'))} action=login") # vuln-code-snippet safe-line ruby_loginj_id_integer_only
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_id_integer_only
