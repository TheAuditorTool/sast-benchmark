require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_broadcast_taint
def log_broadcast_taint(req)
  [LOGGER, Logger.new($stderr)].each { |l| l.info("Event: #{req.param('event')}") } # vuln-code-snippet vuln-line ruby_loginj_broadcast_taint
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_broadcast_taint
