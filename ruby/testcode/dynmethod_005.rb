require_relative 'shared'

class UserRecord
  attr_accessor :name, :email, :role
  def initialize
    @name  = 'default'
    @email = 'default@example.com'
    @role  = 'user'
  end
end

# vuln-code-snippet start ruby_dynmethod_ivar_set
def dynmethod_ivar_set(req)
  field = req.param('field')
  value = req.param('value')
  record = UserRecord.new
  record.instance_variable_set("@#{field}", value) # vuln-code-snippet vuln-line ruby_dynmethod_ivar_set
  BenchmarkResponse.ok(record.to_s)
end
# vuln-code-snippet end ruby_dynmethod_ivar_set
