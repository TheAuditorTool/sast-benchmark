require_relative 'shared'

require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_capture2_str
def grep_logs(req)
  pattern = req.param('pattern')
  stdout, _status = Open3.capture2("grep #{pattern} /var/log/app.log")  # vuln-code-snippet vuln-line ruby_cmdi_open3_capture2_str
  BenchmarkResponse.ok(stdout)
end
# vuln-code-snippet end ruby_cmdi_open3_capture2_str
