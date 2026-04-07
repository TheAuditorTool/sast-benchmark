require_relative 'shared'

SAFE_DIR = '/app/files'.freeze

def read_basename_join(req)
  path = File.join(SAFE_DIR, File.basename(req.param('file')))
  File.read(path)
  BenchmarkResponse.ok(File.read(path))
end
