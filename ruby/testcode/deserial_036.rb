require_relative 'shared'
require 'dry-struct'

module Types
  include Dry.Types()
end

class UserSchema < Dry::Struct
  attribute :name, Types::String
  attribute :age,  Types::Integer
end

# vuln-code-snippet start ruby_deser_dry_struct_from_json
def dry_struct_from_json_handler(req)
  data = JSON.parse(req.post('data'))  # vuln-code-snippet safe-line ruby_deser_dry_struct_from_json
  obj = UserSchema.new(data)
  BenchmarkResponse.json({ result: obj.to_h })
end
# vuln-code-snippet end ruby_deser_dry_struct_from_json
