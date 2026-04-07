require_relative 'shared'

def load_and_create(req)
  gem_name = req.param('gem')
  class_name = req.param('class')
  require gem_name
  klass = Object.const_get(class_name)
  BenchmarkResponse.ok(klass.new.to_s)
end
