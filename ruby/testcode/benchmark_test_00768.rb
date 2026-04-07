require_relative 'shared'
require 'dry-struct'

module Types
  include Dry.Types()
end

class UserSchema < Dry::Struct
  attribute :name, Types::String
  attribute :age,  Types::Integer
end

def dry_struct_from_json_handler(req)
  data = JSON.parse(req.post('data'))
  obj = UserSchema.new(data)
  BenchmarkResponse.json({ result: obj.to_h })
end
