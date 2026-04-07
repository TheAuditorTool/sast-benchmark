require_relative 'shared'

require 'open3'

def grep_logs(req)
  pattern = req.param('pattern')
  stdout, _status = Open3.capture2("grep #{pattern} /var/log/app.log")
  BenchmarkResponse.ok(stdout)
end
