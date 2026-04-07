require_relative 'shared'

class ReportService
  def generate(data) = "report:#{data}"
  def export(data)   = "export:#{data}"
end

def reflect_send(req)
  method_name = req.param('action')
  arg = req.param('data')
  obj = ReportService.new
  result = obj.send(method_name, arg)
  BenchmarkResponse.ok(result.to_s)
end
