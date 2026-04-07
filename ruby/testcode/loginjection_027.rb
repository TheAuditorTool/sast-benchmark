require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_elk_tag_inject
def log_elk_tag_inject(req)
  LOGGER.info("action=#{req.param('cmd')} status=ok user=#{req.param('user')}") # vuln-code-snippet vuln-line ruby_loginj_elk_tag_inject
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_elk_tag_inject
