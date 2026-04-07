require_relative 'shared'

FIXED_SCHEMA = { 'type' => 'object', 'properties' => { 'name' => { 'type' => 'string' } } }.freeze

# vuln-code-snippet start ruby_deser_yaml_schema_validate
def yaml_schema_validate_handler(req)
  raw = req.post('data')
  parsed = YAML.safe_load(raw, permitted_classes: [])  # vuln-code-snippet safe-line ruby_deser_yaml_schema_validate
  JSON::Validator.validate!(FIXED_SCHEMA, parsed)
  BenchmarkResponse.json({ result: parsed })
end
# vuln-code-snippet end ruby_deser_yaml_schema_validate
