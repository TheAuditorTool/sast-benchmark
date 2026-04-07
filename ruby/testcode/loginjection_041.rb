require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_lograge_config
def log_lograge_config(req)
  LOGGER.info({ path: req.header('REQUEST_URI'), status: 200 }.to_json) # vuln-code-snippet safe-line ruby_loginj_lograge_config
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_lograge_config
