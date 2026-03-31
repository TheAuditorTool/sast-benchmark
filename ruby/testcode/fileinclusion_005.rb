require_relative 'shared'

# vuln-code-snippet start ruby_fi_require_relative_taint
def activate_extension(req)
  ext_name = req.param('extension')
  require_relative("extensions/#{ext_name}") # vuln-code-snippet vuln-line ruby_fi_require_relative_taint
  BenchmarkResponse.ok("extension activated")
end
# vuln-code-snippet end ruby_fi_require_relative_taint
