require_relative '../../../testcode/shared'
require 'yaml'

# Rails API - AdminController
# Covers: cmdi, pathtraver, deserial

# vuln-code-snippet start ra_cmdi_backtick
def admin_run_command(req)
  tool = req.param('tool')
  output = `#{tool}` # vuln-code-snippet vuln-line ra_cmdi_backtick
  BenchmarkResponse.json({ output: output })
end
# vuln-code-snippet end ra_cmdi_backtick

# vuln-code-snippet start ra_cmdi_system_array
def admin_run_tool(req)
  tool_name = req.param('tool')
  allowed_tools = %w[uptime hostname date]
  return BenchmarkResponse.bad_request('tool not allowed') unless allowed_tools.include?(tool_name)
  system([tool_name, tool_name]) # vuln-code-snippet safe-line ra_cmdi_system_array
  BenchmarkResponse.json({ status: 'executed' })
end
# vuln-code-snippet end ra_cmdi_system_array

# vuln-code-snippet start ra_pt_send_file
def admin_download_file(req)
  filename = req.param('file')
  content = File.read("/var/app/reports/#{filename}") # vuln-code-snippet vuln-line ra_pt_send_file
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ra_pt_send_file

# vuln-code-snippet start ra_pt_basename
def admin_download_safe(req)
  filename = req.param('file')
  safe_name = File.basename(filename) # vuln-code-snippet safe-line ra_pt_basename
  content = File.read("/var/app/reports/#{safe_name}")
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ra_pt_basename

# vuln-code-snippet start ra_deser_yaml_load
def admin_load_config(req)
  user_config = req.post('config')
  config = YAML.load(user_config) # vuln-code-snippet vuln-line ra_deser_yaml_load
  BenchmarkResponse.json({ config: config })
end
# vuln-code-snippet end ra_deser_yaml_load

# vuln-code-snippet start ra_deser_yaml_safe
def admin_load_config_safe(req)
  user_config = req.post('config')
  config = YAML.safe_load(user_config) # vuln-code-snippet safe-line ra_deser_yaml_safe
  BenchmarkResponse.json({ config: config })
end
# vuln-code-snippet end ra_deser_yaml_safe
