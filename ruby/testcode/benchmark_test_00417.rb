require_relative 'shared'

def read_log(req)
  log_name = req.param('log')
  lines = File.readlines("/var/log/#{log_name}")
  BenchmarkResponse.ok(lines.join)
end
