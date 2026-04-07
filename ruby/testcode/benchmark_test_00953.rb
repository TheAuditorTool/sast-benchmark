require_relative 'shared'

FIXED_SCHEMA = { 'type' => 'object', 'properties' => { 'name' => { 'type' => 'string' } } }.freeze

def yaml_schema_validate_handler(req)
  raw = req.post('data')
  parsed = YAML.safe_load(raw, permitted_classes: [])
  JSON::Validator.validate!(FIXED_SCHEMA, parsed)
  BenchmarkResponse.json({ result: parsed })
end
