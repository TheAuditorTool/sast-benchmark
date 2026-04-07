require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_struct_member_set
def struct_member_dispatch(req)
  field = req.param('field')
  val   = req.param('val')
  s = Struct.new(field.to_sym).new              # vuln-code-snippet vuln-line ruby_dynmethod_struct_member_set
  s.send("#{field}=", val)
  BenchmarkResponse.json({ result: s.to_s })
end
# vuln-code-snippet end ruby_dynmethod_struct_member_set
