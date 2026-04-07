require_relative 'shared'

# vuln-code-snippet start ruby_pt_no_user_in_path
def read_server_side_path(req)
  record_id = Integer(req.param('id'))
  filename = "report_#{record_id}.pdf"
  BenchmarkResponse.ok(File.read(File.join('/app/reports', filename))) # vuln-code-snippet safe-line ruby_pt_no_user_in_path
end
# vuln-code-snippet end ruby_pt_no_user_in_path
