require_relative 'shared'

def run_remote_command(req)
  cmd = req.param('cmd')
  host = 'internal.example.com'
  user = 'deploy'
  result = ''
  Net::SSH.start(host, user) do |ssh|
    result = ssh.exec!("#{cmd}")
  end
  BenchmarkResponse.ok(result)
end
