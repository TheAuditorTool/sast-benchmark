require_relative 'shared'

def build_scan_command(target)
  "nmap -sV " + target
end

# vuln-code-snippet start ruby_cmdi_multihop
def run_network_scan(req)
  target = req.param('target')
  cmd = build_scan_command(target)
  output = `#{cmd}` # vuln-code-snippet vuln-line ruby_cmdi_multihop
  BenchmarkResponse.ok(output)
end
# vuln-code-snippet end ruby_cmdi_multihop
