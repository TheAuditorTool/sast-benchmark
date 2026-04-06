require_relative 'shared'

# vuln-code-snippet start ruby_fi_autoload_user_class
def autoload_class(req)
  class_name = req.param('class')
  path = req.param('path')
  Object.autoload(class_name.to_sym, path) # vuln-code-snippet vuln-line ruby_fi_autoload_user_class
  BenchmarkResponse.ok("autoloaded #{class_name}")
end
# vuln-code-snippet end ruby_fi_autoload_user_class
