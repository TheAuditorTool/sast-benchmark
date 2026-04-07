require_relative 'shared'

require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_capture2_array
def grep_logs_safe(req)
  pattern = req.param('pattern')
  logfile = '/var/log/app.log'
  stdout, _status = Open3.capture2('grep', '--', Regexp.escape(pattern), logfile)  # vuln-code-snippet safe-line ruby_cmdi_open3_capture2_array
  BenchmarkResponse.ok(stdout)
end
# vuln-code-snippet end ruby_cmdi_open3_capture2_array
