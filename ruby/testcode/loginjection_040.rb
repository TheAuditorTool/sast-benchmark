require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_json_encode_value
def log_json_encode_value(req)
  LOGGER.info(JSON.generate({ event: req.param('action'), user: 'system' })) # vuln-code-snippet safe-line ruby_loginj_json_encode_value
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_json_encode_value
