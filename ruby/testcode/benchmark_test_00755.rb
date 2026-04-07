require_relative 'shared'

require 'open3'

def find_files_safe(req)
  raw_path = req.param('path')
  validated_path = File.expand_path(raw_path, '/app/data')
  raise 'path traversal' unless validated_path.start_with?('/app/data')
  output, _status = Open3.capture2e('find', validated_path, '-type', 'f')
  BenchmarkResponse.ok(output)
end
