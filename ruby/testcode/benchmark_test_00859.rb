require_relative 'shared'

def read_expand_path_start_with(req)
  base = '/app/files'
  user_rel = req.param('path')
  p = File.expand_path(user_rel, base)
  raise unless p.start_with?(base + '/')
  BenchmarkResponse.ok(File.read(p))
end
