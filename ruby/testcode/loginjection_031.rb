require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_json_key_inject
def log_json_key_inject(req)
  LOGGER.info({ req.param('key') => 'value' }.to_json) # vuln-code-snippet vuln-line ruby_loginj_json_key_inject
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_json_key_inject
