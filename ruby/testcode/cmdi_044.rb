require_relative 'shared'

ALLOWED_SSH_CMDS = {
  'restart' => 'sudo systemctl restart app',
  'status'  => 'sudo systemctl status app',
  'reload'  => 'sudo systemctl reload app'
}.freeze

# vuln-code-snippet start ruby_cmdi_net_ssh_safe
def run_remote_safe(req)
  action = req.param('action')
  host = 'internal.example.com'
  user = 'deploy'
  result = ''
  Net::SSH.start(host, user) do |ssh|
    result = ssh.exec!(ALLOWED_SSH_CMDS.fetch(action))  # vuln-code-snippet safe-line ruby_cmdi_net_ssh_safe
  end
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_net_ssh_safe
