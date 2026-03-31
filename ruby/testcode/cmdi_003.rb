require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_system_concat
def ping_host(req)
  host = req.param('host')
  result = system("ping -c 3 " + host) # vuln-code-snippet vuln-line ruby_cmdi_system_concat
  status = result ? "host reachable" : "host unreachable"
  BenchmarkResponse.ok(status)
end
# vuln-code-snippet end ruby_cmdi_system_concat
