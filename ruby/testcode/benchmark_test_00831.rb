require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

def tap_dispatch(req)
  setter = req.param('setter')
  val    = req.param('val')
  record = OpenStruct.new(name: 'x')
  record.tap { |r| r.send(setter, val) }
  BenchmarkResponse.json({ result: "ok" })
end
