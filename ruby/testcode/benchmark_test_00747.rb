require_relative 'shared'
require 'syslog'

def log_syslog_format(req)
  Syslog.open('app', Syslog::LOG_PID, Syslog::LOG_USER)
  Syslog.log(Syslog::LOG_INFO, "Login: #{req.param('user')}")
  Syslog.close
  BenchmarkResponse.json({ ok: true })
end
