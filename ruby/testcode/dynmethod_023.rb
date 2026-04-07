require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

# vuln-code-snippet start ruby_dynmethod_ivar_set_model
def set_user_attribute(req)
  user = OpenStruct.new
  attr = req.param('attr')
  val  = req.param('val')
  user.instance_variable_set("@#{attr}", val) # vuln-code-snippet vuln-line ruby_dynmethod_ivar_set_model
  BenchmarkResponse.json({ result: "set" })
end
# vuln-code-snippet end ruby_dynmethod_ivar_set_model
