require_relative 'shared'
require 'syslog'

# vuln-code-snippet start ruby_loginj_syslog_sprintf
def log_syslog_sprintf(req)
  Syslog.open('app', Syslog::LOG_PID, Syslog::LOG_USER)
  Syslog.log(Syslog::LOG_INFO, '%s', req.param('msg').gsub(/[\r\n]/, '')) # vuln-code-snippet safe-line ruby_loginj_syslog_sprintf
  Syslog.close
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_syslog_sprintf
