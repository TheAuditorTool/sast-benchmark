require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_custom_formatter
def log_custom_formatter(req)
  LOGGER.formatter = proc { |s, t, p, m| "#{t} #{req.param('context')}: #{m}\n" } # vuln-code-snippet vuln-line ruby_loginj_custom_formatter
  LOGGER.info("processed")
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_custom_formatter
