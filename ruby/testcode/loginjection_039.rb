require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_gsub_control_chars
def log_gsub_control_chars(req)
  sanitized = req.param('val').gsub(/[[:cntrl:]]/, '?')
  LOGGER.info(sanitized) # vuln-code-snippet safe-line ruby_loginj_gsub_control_chars
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_gsub_control_chars
