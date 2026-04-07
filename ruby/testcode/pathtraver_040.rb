require_relative 'shared'

# vuln-code-snippet start ruby_pt_send_file_basename
def send_file_basename(req)
  fname = File.basename(req.param('name'))
  data = File.read(File.join('/var/uploads', fname))
  BenchmarkResponse.new(200, { 'Content-Type' => 'application/octet-stream' }, [data]) # vuln-code-snippet safe-line ruby_pt_send_file_basename
end
# vuln-code-snippet end ruby_pt_send_file_basename
