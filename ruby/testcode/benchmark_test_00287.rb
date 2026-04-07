require_relative 'shared'

require 'open3'

def grep_logs_safe(req)
  pattern = req.param('pattern')
  logfile = '/var/log/app.log'
  stdout, _status = Open3.capture2('grep', '--', Regexp.escape(pattern), logfile)
  BenchmarkResponse.ok(stdout)
end
