require_relative 'shared'

def list_directory(req)
  dir = req.param('dir')
  Kernel.exec("ls -la #{dir}")
  BenchmarkResponse.ok('done')
end
