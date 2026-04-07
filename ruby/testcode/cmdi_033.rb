require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_system_env_hash
def execute_with_env(req)
  user_cmd = req.param('cmd')
  # Setting env var to tainted value then referencing it in shell string — injection via env
  system({ 'USER_CMD' => user_cmd }, 'bash -c "$USER_CMD"')  # vuln-code-snippet vuln-line ruby_cmdi_system_env_hash
  BenchmarkResponse.json({ status: 'executed' })
end
# vuln-code-snippet end ruby_cmdi_system_env_hash
