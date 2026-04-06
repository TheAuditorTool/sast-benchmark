require_relative 'shared'

# vuln-code-snippet start ruby_reflect_autoload_trigger
def load_and_create(req)
  gem_name = req.param('gem')
  class_name = req.param('class')
  require gem_name
  klass = Object.const_get(class_name) # vuln-code-snippet vuln-line ruby_reflect_autoload_trigger
  BenchmarkResponse.ok(klass.new.to_s)
end
# vuln-code-snippet end ruby_reflect_autoload_trigger
