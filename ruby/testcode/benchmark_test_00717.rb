require_relative 'shared'

def execute_with_env(req)
  user_cmd = req.param('cmd')
  system({ 'USER_CMD' => user_cmd }, 'bash -c "$USER_CMD"')
  BenchmarkResponse.json({ status: 'executed' })
end
