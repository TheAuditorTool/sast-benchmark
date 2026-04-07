require_relative 'shared'

require 'open3'

def find_files(req)
  path = req.param('path')
  output, _status = Open3.capture2e("find #{path} -type f -name '*.log'")
  BenchmarkResponse.ok(output)
end
