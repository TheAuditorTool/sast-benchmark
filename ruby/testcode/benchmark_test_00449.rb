require_relative 'shared'

def get_server_status(req)
  raw_output = `systemctl status nginx`
  lines = raw_output.lines.first(10).join
  BenchmarkResponse.ok(lines)
end
