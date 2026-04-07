require_relative 'shared'

require 'open3'

def count_lines_safe(req)
  filename = req.param('file').gsub(/[^a-z0-9._\-]/i, '')
  validated_file = File.join('/app/uploads', filename)
  stdout, _stderr, _status = Open3.capture3('wc', '-l', validated_file)
  BenchmarkResponse.ok(stdout.strip)
end
