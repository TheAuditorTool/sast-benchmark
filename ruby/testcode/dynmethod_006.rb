require_relative 'shared'

class ProfileRecord
  attr_accessor :name, :bio
  def initialize
    @name = 'default'
    @bio  = ''
  end
end

FIELD_MAP = { 'name' => :@name, 'bio' => :@bio }.freeze

# vuln-code-snippet start ruby_dynmethod_ivar_map
def dynmethod_ivar_map(req)
  field = req.param('field')
  ivar = FIELD_MAP[field] # vuln-code-snippet safe-line ruby_dynmethod_ivar_map
  return BenchmarkResponse.bad_request('invalid field') unless ivar
  record = ProfileRecord.new
  record.instance_variable_set(ivar, req.param('value'))
  BenchmarkResponse.ok(record.to_s)
end
# vuln-code-snippet end ruby_dynmethod_ivar_map
