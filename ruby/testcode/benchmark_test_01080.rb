require_relative 'shared'
require 'pathname'

def read_cleanpath_check(req)
  base = '/app/files'
  p = Pathname.new(base).join(req.param('rel')).cleanpath
  raise 'traversal' unless p.to_s.start_with?(base)
  p.read
  BenchmarkResponse.ok(p.read)
end
