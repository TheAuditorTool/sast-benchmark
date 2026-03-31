require_relative 'shared'

class ReportService
  def generate(data) = "report:#{data}"
  def export(data)   = "export:#{data}"
end

# vuln-code-snippet start ruby_reflect_send
def reflect_send(req)
  method_name = req.param('action')
  arg = req.param('data')
  obj = ReportService.new
  result = obj.send(method_name, arg) # vuln-code-snippet vuln-line ruby_reflect_send
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_reflect_send
