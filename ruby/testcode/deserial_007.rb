require_relative 'shared'
require 'yaml'

# vuln-code-snippet start ruby_deser_yaml_unsafe
def import_data(req)
  data = req.body_str
  parsed = YAML.unsafe_load(data) # vuln-code-snippet vuln-line ruby_deser_yaml_unsafe
  BenchmarkResponse.ok(parsed.to_s)
end
# vuln-code-snippet end ruby_deser_yaml_unsafe
