require_relative 'shared'

def list_directory_safe(req)
  dir = req.param('dir')
  Kernel.exec('ls', '-la', '--', dir)
  BenchmarkResponse.ok('done')
end
