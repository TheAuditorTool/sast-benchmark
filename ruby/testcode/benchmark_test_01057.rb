require_relative 'shared'

def ping_host_safe(req)
  host = req.param('host').gsub(/[^a-z0-9.\-]/i, '')
  pid = Process.spawn('ping', '-c', '1', host)
  Process.wait(pid)
  BenchmarkResponse.json({ pid: pid })
end
