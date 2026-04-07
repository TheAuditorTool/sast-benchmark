require_relative 'shared'
require 'open3'

def search_logs_safe(req)
  pattern = req.param('pattern')
  stdout, _stderr, _status = Open3.capture2("grep", pattern, "/var/log/app.log")
  BenchmarkResponse.ok(stdout)
end
