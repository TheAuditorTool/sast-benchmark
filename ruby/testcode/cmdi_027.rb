require_relative 'shared'

require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_capture3_str
def count_lines(req)
  filename = req.param('file')
  stdout, _stderr, _status = Open3.capture3("wc -l #{filename}")  # vuln-code-snippet vuln-line ruby_cmdi_open3_capture3_str
  BenchmarkResponse.ok(stdout.strip)
end
# vuln-code-snippet end ruby_cmdi_open3_capture3_str
