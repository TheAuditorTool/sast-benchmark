require_relative 'shared'

# vuln-code-snippet start ruby_pt_readlines
def read_log(req)
  log_name = req.param('log')
  lines = File.readlines("/var/log/#{log_name}") # vuln-code-snippet vuln-line ruby_pt_readlines
  BenchmarkResponse.ok(lines.join)
end
# vuln-code-snippet end ruby_pt_readlines
