require_relative 'shared'

require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_capture2e_array
def find_files_safe(req)
  raw_path = req.param('path')
  validated_path = File.expand_path(raw_path, '/app/data')
  raise 'path traversal' unless validated_path.start_with?('/app/data')
  output, _status = Open3.capture2e('find', validated_path, '-type', 'f')  # vuln-code-snippet safe-line ruby_cmdi_open3_capture2e_array
  BenchmarkResponse.ok(output)
end
# vuln-code-snippet end ruby_cmdi_open3_capture2e_array
