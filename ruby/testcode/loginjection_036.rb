require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_strip_crlf_null
def log_strip_crlf_null(req)
  msg = req.param('msg').gsub(/[\r\n\x00]/, ' ')
  LOGGER.info(msg) # vuln-code-snippet safe-line ruby_loginj_strip_crlf_null
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_strip_crlf_null
