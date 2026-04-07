require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_structured_hash
def log_structured_hash(req)
  LOGGER.info({ user_id: 42, action: 'login', ip: '127.0.0.1' }.to_json) # vuln-code-snippet safe-line ruby_loginj_structured_hash
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_structured_hash
