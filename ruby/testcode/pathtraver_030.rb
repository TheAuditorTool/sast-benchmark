require_relative 'shared'

# vuln-code-snippet start ruby_pt_send_data_file
def send_data_file(req)
  data = File.read(req.param('path')) # vuln-code-snippet vuln-line ruby_pt_send_data_file
  BenchmarkResponse.new(200, { 'Content-Type' => 'application/octet-stream' }, [data])
end
# vuln-code-snippet end ruby_pt_send_data_file
