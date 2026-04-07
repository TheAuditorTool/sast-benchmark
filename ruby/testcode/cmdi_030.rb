require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_net_ssh_exec
def run_remote_command(req)
  cmd = req.param('cmd')
  host = 'internal.example.com'
  user = 'deploy'
  result = ''
  Net::SSH.start(host, user) do |ssh|
    result = ssh.exec!("#{cmd}")  # vuln-code-snippet vuln-line ruby_cmdi_net_ssh_exec
  end
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_net_ssh_exec
