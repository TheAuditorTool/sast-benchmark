require_relative 'shared'

def glob_files(req)
  matches = Dir.glob(req.param('pattern'))
  BenchmarkResponse.json({ matches: matches })
end
