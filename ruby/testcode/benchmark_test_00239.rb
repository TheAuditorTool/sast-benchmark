require_relative 'shared'

def glob_fixed_pattern(req)
  files = Dir.glob('/app/data/*.csv')
  safe_prefix = req.param('prefix').gsub(/[^a-z0-9]/, '')
  result = files.select { |f| File.basename(f).start_with?(safe_prefix) }
  BenchmarkResponse.json({ files: result })
end
