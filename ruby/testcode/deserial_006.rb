require_relative 'shared'
require 'yaml'
require 'date'

# vuln-code-snippet start ruby_deser_yaml_permitted
def load_schedule(req)
  input = req.body_str
  data = YAML.safe_load(input, permitted_classes: [Symbol, Date]) # vuln-code-snippet safe-line ruby_deser_yaml_permitted
  event = data.fetch('event', 'none').to_s
  BenchmarkResponse.ok("event: #{event}")
end
# vuln-code-snippet end ruby_deser_yaml_permitted
