require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_hash_only_params
def log_hash_only_params(req)
  LOGGER.info({ timestamp: Time.now.to_i, path: '/api/data', method: 'GET' }.to_json) # vuln-code-snippet safe-line ruby_loginj_hash_only_params
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_hash_only_params
