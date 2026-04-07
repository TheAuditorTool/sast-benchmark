require_relative 'shared'

def read_server_side_path(req)
  record_id = Integer(req.param('id'))
  filename = "report_#{record_id}.pdf"
  BenchmarkResponse.ok(File.read(File.join('/app/reports', filename)))
end
