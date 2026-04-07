require_relative 'shared'

require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_capture2e_str
def find_files(req)
  path = req.param('path')
  output, _status = Open3.capture2e("find #{path} -type f -name '*.log'")  # vuln-code-snippet vuln-line ruby_cmdi_open3_capture2e_str
  BenchmarkResponse.ok(output)
end
# vuln-code-snippet end ruby_cmdi_open3_capture2e_str
