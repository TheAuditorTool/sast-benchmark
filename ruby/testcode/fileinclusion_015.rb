require_relative 'shared'

# vuln-code-snippet start ruby_fi_gem_require
def require_gem(req)
  gem_name = req.param('gem')
  require gem_name # vuln-code-snippet vuln-line ruby_fi_gem_require
  BenchmarkResponse.ok("loaded #{gem_name}")
end
# vuln-code-snippet end ruby_fi_gem_require
