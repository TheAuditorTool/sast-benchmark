require_relative 'shared'
require 'pathname'

def read_pathname(req)
  content = Pathname.new(req.param('path')).read
  BenchmarkResponse.ok(content)
end
