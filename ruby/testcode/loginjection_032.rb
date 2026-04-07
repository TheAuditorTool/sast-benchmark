require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_multiline_param
def log_multiline_param(req)
  LOGGER.debug("Processing request: #{req.param('body')}") # vuln-code-snippet vuln-line ruby_loginj_multiline_param
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_multiline_param
