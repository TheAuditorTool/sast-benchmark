require_relative 'shared'
require 'yaml'

# vuln-code-snippet start ruby_deser_yaml_safe_load
def load_config(req)
  input = req.body_str
  config = YAML.safe_load(input) # vuln-code-snippet safe-line ruby_deser_yaml_safe_load
  name = config.fetch('name', 'default').to_s
  BenchmarkResponse.ok("config loaded: #{name}")
end
# vuln-code-snippet end ruby_deser_yaml_safe_load
