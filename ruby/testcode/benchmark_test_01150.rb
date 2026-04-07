require_relative 'shared'

def stat_file(req)
  size = File.stat(req.param('file')).size
  BenchmarkResponse.json({ size: size })
end
