require_relative 'shared'

def import_module(req)
  module_path = req.param('module')
  require(module_path)
  BenchmarkResponse.ok("module imported")
end
