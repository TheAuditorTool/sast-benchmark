require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_progname_taint
def log_progname_taint(req)
  Logger.new($stdout, progname: req.param('service')).info("started") # vuln-code-snippet vuln-line ruby_loginj_progname_taint
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_progname_taint
