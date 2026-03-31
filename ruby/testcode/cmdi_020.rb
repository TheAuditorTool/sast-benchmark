require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_constant_cmd
def get_server_status(req)
  raw_output = `systemctl status nginx` # vuln-code-snippet safe-line ruby_cmdi_constant_cmd
  lines = raw_output.lines.first(10).join
  BenchmarkResponse.ok(lines)
end
# vuln-code-snippet end ruby_cmdi_constant_cmd
