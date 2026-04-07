require_relative 'shared'

def autoload_class(req)
  class_name = req.param('class')
  path = req.param('path')
  Object.autoload(class_name.to_sym, path)
  BenchmarkResponse.ok("autoloaded #{class_name}")
end
