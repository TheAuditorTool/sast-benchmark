require_relative 'shared'

# vuln-code-snippet start ruby_fi_require_user
def import_module(req)
  module_path = req.param('module')
  require(module_path) # vuln-code-snippet vuln-line ruby_fi_require_user
  BenchmarkResponse.ok("module imported")
end
# vuln-code-snippet end ruby_fi_require_user
