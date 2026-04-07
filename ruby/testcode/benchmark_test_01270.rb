require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

def set_user_attribute(req)
  user = OpenStruct.new
  attr = req.param('attr')
  val  = req.param('val')
  user.instance_variable_set("@#{attr}", val)
  BenchmarkResponse.json({ result: "set" })
end
