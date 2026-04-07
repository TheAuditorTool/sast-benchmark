require_relative 'shared'

class UserRecord
  attr_accessor :name, :email, :role
  def initialize
    @name  = 'default'
    @email = 'default@example.com'
    @role  = 'user'
  end
end

def handler(req)
  field = req.param('field')
  value = req.param('value')
  record = UserRecord.new
  record.instance_variable_set("@#{field}", value)
  BenchmarkResponse.ok(record.to_s)
end
