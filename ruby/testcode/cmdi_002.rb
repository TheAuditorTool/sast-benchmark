require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_system_array
def list_dir_safe(req)
  dir = req.param('dir')
  success = system("ls", "-la", dir) # vuln-code-snippet safe-line ruby_cmdi_system_array
  result = success ? "listing complete" : "command failed"
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_system_array
