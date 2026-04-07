require_relative 'shared'

require 'open3'

def count_lines(req)
  filename = req.param('file')
  stdout, _stderr, _status = Open3.capture3("wc -l #{filename}")
  BenchmarkResponse.ok(stdout.strip)
end
