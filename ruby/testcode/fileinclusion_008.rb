require_relative 'shared'

# vuln-code-snippet start ruby_fi_load_validated
def load_safe_plugin(req)
  name = req.param('plugin')
  unless name.match?(/\A[a-z_]+\z/) && !name.include?('..') # vuln-code-snippet safe-line ruby_fi_load_validated
    return BenchmarkResponse.bad_request("invalid plugin name")
  end
  load("plugins/#{name}.rb")
  BenchmarkResponse.ok("plugin loaded")
end
# vuln-code-snippet end ruby_fi_load_validated
