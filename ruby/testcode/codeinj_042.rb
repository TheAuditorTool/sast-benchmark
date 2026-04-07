require_relative 'shared'

begin
  require 'json-schema'
rescue LoadError
end

USER_SCHEMA = {
  'type' => 'object',
  'properties' => {
    'name' => { 'type' => 'string' },
    'age'  => { 'type' => 'integer', 'minimum' => 0 }
  },
  'required' => ['name']
}.freeze

# vuln-code-snippet start ruby_codeinj_json_schema_validate
def validate_user_data(req)
  raw = req.param('data')
  parsed = JSON.parse(raw)
  JSON::Validator.validate!(USER_SCHEMA, parsed) # vuln-code-snippet safe-line ruby_codeinj_json_schema_validate
  BenchmarkResponse.json({ valid: true })
end
# vuln-code-snippet end ruby_codeinj_json_schema_validate
