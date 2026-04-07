require_relative 'shared'

SAFE_ROOT = '/app/files'.freeze

def read_with_chdir(req)
  fname = File.basename(req.param('file'))
  content = Dir.chdir(SAFE_ROOT) { File.read(fname) }
  BenchmarkResponse.ok(content)
end
