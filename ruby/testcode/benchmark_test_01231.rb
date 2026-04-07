require_relative 'shared'

def ping_host(req)
  host = req.param('host')
  pid = Process.spawn("ping -c 4 #{host}")
  Process.wait(pid)
  BenchmarkResponse.json({ pid: pid })
end
