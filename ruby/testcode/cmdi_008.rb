require_relative 'shared'
require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_array
def search_logs_safe(req)
  pattern = req.param('pattern')
  stdout, _stderr, _status = Open3.capture2("grep", pattern, "/var/log/app.log") # vuln-code-snippet safe-line ruby_cmdi_open3_array
  BenchmarkResponse.ok(stdout)
end
# vuln-code-snippet end ruby_cmdi_open3_array
