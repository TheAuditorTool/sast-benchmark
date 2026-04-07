require_relative 'shared'

ALLOWED_PING_HOSTS = %w[8.8.8.8 1.1.1.1 192.168.1.1].freeze

def ping_allowlisted_host(req)
  host = req.param('host')
  safe_host = ALLOWED_PING_HOSTS.include?(host) ? host : '127.0.0.1'
  pid = spawn({}, 'ping', '-c', '1', safe_host)
  Process.wait(pid)
  BenchmarkResponse.json({ host: safe_host })
end
