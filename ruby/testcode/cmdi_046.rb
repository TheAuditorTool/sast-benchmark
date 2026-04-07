require_relative 'shared'

ALLOWED_PING_HOSTS = %w[8.8.8.8 1.1.1.1 192.168.1.1].freeze

# vuln-code-snippet start ruby_cmdi_spawn_env_safe
def ping_allowlisted_host(req)
  host = req.param('host')
  safe_host = ALLOWED_PING_HOSTS.include?(host) ? host : '127.0.0.1'
  # Empty env hash {} prevents env injection; array args bypass shell
  pid = spawn({}, 'ping', '-c', '1', safe_host)  # vuln-code-snippet safe-line ruby_cmdi_spawn_env_safe
  Process.wait(pid)
  BenchmarkResponse.json({ host: safe_host })
end
# vuln-code-snippet end ruby_cmdi_spawn_env_safe
