require_relative 'shared'
require 'yaml'

# vuln-code-snippet start ruby_deser_yaml_load
def apply_user_config(req)
  config_str = req.body_str
  settings = YAML.load(config_str) # vuln-code-snippet vuln-line ruby_deser_yaml_load
  BenchmarkResponse.ok(settings.to_s)
end
# vuln-code-snippet end ruby_deser_yaml_load
