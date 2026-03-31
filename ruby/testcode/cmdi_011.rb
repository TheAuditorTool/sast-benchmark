require_relative 'shared'
require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_concat
def resolve_hostname(req)
  hostname = req.param('hostname')
  stdout, _stderr, _status = Open3.capture2("nslookup " + hostname) # vuln-code-snippet vuln-line ruby_cmdi_open3_concat
  BenchmarkResponse.ok(stdout)
end
# vuln-code-snippet end ruby_cmdi_open3_concat
