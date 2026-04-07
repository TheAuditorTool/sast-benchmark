require_relative 'shared'

def list_dir_entries(req)
  entries = Dir.entries(req.param('dir'))
  BenchmarkResponse.json({ entries: entries })
end
