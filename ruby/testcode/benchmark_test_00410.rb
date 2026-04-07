require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

ATTR_MAP_043 = { 'name' => :name, 'bio' => :bio }.freeze

def ivar_map_safe(req)
  user = OpenStruct.new(name: 'alice', bio: 'developer')
  m    = ATTR_MAP_043.fetch(req.param('field'))
  result = user.send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
