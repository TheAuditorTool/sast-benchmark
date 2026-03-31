require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_env_path
def run_build_tool(req)
  tool_path = req.header('X-Tool-Path')
  ENV['PATH'] = tool_path + ":" + ENV['PATH']
  result = `make all` # vuln-code-snippet vuln-line ruby_cmdi_env_path
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_env_path
