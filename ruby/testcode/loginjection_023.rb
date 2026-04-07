require_relative 'shared'
require 'syslog'

# vuln-code-snippet start ruby_loginj_syslog_format
def log_syslog_format(req)
  Syslog.open('app', Syslog::LOG_PID, Syslog::LOG_USER)
  Syslog.log(Syslog::LOG_INFO, "Login: #{req.param('user')}") # vuln-code-snippet vuln-line ruby_loginj_syslog_format
  Syslog.close
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_syslog_format
