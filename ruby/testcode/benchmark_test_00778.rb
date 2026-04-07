require_relative 'shared'

def eval_plugin(req)
  path = req.param('path')
  eval(File.read(path))
  BenchmarkResponse.ok('loaded')
end
