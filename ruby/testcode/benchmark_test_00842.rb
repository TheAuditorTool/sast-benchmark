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

def validate_user_data(req)
  raw = req.param('data')
  parsed = JSON.parse(raw)
  JSON::Validator.validate!(USER_SCHEMA, parsed)
  BenchmarkResponse.json({ valid: true })
end
