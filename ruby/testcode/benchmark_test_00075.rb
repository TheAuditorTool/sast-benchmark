require_relative 'shared'
require 'pathname'

def read_no_dotdot(req)
  joined = File.join('/app/files', req.param('rel'))
  raise if Pathname.new(joined).each_filename.any? { |f| f == '..' }
  BenchmarkResponse.ok(File.read(joined))
end
