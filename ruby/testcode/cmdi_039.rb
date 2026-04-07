require_relative 'shared'

require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_capture3_array
def count_lines_safe(req)
  filename = req.param('file').gsub(/[^a-z0-9._\-]/i, '')
  validated_file = File.join('/app/uploads', filename)
  stdout, _stderr, _status = Open3.capture3('wc', '-l', validated_file)  # vuln-code-snippet safe-line ruby_cmdi_open3_capture3_array
  BenchmarkResponse.ok(stdout.strip)
end
# vuln-code-snippet end ruby_cmdi_open3_capture3_array
