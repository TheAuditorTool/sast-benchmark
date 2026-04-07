require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

# vuln-code-snippet start ruby_dynmethod_tap_send
def tap_dispatch(req)
  setter = req.param('setter')
  val    = req.param('val')
  record = OpenStruct.new(name: 'x')
  record.tap { |r| r.send(setter, val) } # vuln-code-snippet vuln-line ruby_dynmethod_tap_send
  BenchmarkResponse.json({ result: "ok" })
end
# vuln-code-snippet end ruby_dynmethod_tap_send
