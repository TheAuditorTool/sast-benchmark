require_relative 'shared'

# vuln-code-snippet start ruby_fi_autoload_user
def register_adapter(req)
  name = req.param('name')
  autoload(name.to_sym, "adapters/#{name}") # vuln-code-snippet vuln-line ruby_fi_autoload_user
  BenchmarkResponse.ok("adapter registered")
end
# vuln-code-snippet end ruby_fi_autoload_user
