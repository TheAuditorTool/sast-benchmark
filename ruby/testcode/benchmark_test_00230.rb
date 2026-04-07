require_relative 'shared'

def struct_member_dispatch(req)
  field = req.param('field')
  val   = req.param('val')
  s = Struct.new(field.to_sym).new
  s.send("#{field}=", val)
  BenchmarkResponse.json({ result: s.to_s })
end
