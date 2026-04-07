require_relative 'shared'

def execute_script(req)
  path = req.param('path')
  Kernel.load(path)
  BenchmarkResponse.ok("script executed")
end
