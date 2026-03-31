require_relative 'shared'

# vuln-code-snippet start ruby_fi_load_user_path
def load_plugin(req)
  plugin_name = req.param('plugin')
  load("plugins/#{plugin_name}.rb") # vuln-code-snippet vuln-line ruby_fi_load_user_path
  BenchmarkResponse.ok("plugin loaded")
end
# vuln-code-snippet end ruby_fi_load_user_path
