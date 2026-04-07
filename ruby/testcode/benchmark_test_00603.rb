require_relative 'shared'

def build_scan_command(target)
  "nmap -sV " + target
end

def run_network_scan(req)
  target = req.param('target')
  cmd = build_scan_command(target)
  output = `
  BenchmarkResponse.ok(output)
end
