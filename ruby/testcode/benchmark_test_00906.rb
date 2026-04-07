require_relative 'shared'
require 'syslog'

def log_syslog_sprintf(req)
  Syslog.open('app', Syslog::LOG_PID, Syslog::LOG_USER)
  Syslog.log(Syslog::LOG_INFO, '%s', req.param('msg').gsub(/[\r\n]/, ''))
  Syslog.close
  BenchmarkResponse.json({ ok: true })
end
