require_relative 'shared'

def ping_host(req)
  host = req.param('host')
  result = system("ping -c 3 " + host)
  status = result ? "host reachable" : "host unreachable"
  BenchmarkResponse.ok(status)
end
