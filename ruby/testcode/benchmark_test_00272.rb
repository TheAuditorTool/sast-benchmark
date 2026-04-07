require_relative 'shared'

def read_expanded_no_check(req)
  base = '/app/files'
  path = File.expand_path(req.param('path'), base)
  BenchmarkResponse.ok(File.read(path))
end
