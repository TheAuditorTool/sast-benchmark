require_relative 'shared'

ALLOWED_SSH_CMDS = {
  'restart' => 'sudo systemctl restart app',
  'status'  => 'sudo systemctl status app',
  'reload'  => 'sudo systemctl reload app'
}.freeze

def run_remote_safe(req)
  action = req.param('action')
  host = 'internal.example.com'
  user = 'deploy'
  result = ''
  Net::SSH.start(host, user) do |ssh|
    result = ssh.exec!(ALLOWED_SSH_CMDS.fetch(action))
  end
  BenchmarkResponse.ok(result)
end
