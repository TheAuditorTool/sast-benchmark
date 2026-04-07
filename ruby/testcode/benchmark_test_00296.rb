require_relative 'shared'

class ProfileRecord
  attr_accessor :name, :bio
  def initialize
    @name = 'default'
    @bio  = ''
  end
end

FIELD_MAP = { 'name' => :@name, 'bio' => :@bio }.freeze

def handler(req)
  field = req.param('field')
  ivar = FIELD_MAP[field]
  return BenchmarkResponse.bad_request('invalid field') unless ivar
  record = ProfileRecord.new
  record.instance_variable_set(ivar, req.param('value'))
  BenchmarkResponse.ok(record.to_s)
end
