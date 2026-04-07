require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

ATTR_MAP_043 = { 'name' => :name, 'bio' => :bio }.freeze

# vuln-code-snippet start ruby_dynmethod_ivar_map_safe
def ivar_map_safe(req)
  user = OpenStruct.new(name: 'alice', bio: 'developer')
  m    = ATTR_MAP_043.fetch(req.param('field')) # vuln-code-snippet safe-line ruby_dynmethod_ivar_map_safe
  result = user.send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_ivar_map_safe
