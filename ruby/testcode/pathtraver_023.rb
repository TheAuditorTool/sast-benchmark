require_relative 'shared'

# vuln-code-snippet start ruby_pt_io_binread
def binread_file(req)
  data = IO.binread(req.param('file')) # vuln-code-snippet vuln-line ruby_pt_io_binread
  BenchmarkResponse.new(200, { 'Content-Type' => 'application/octet-stream' }, [data])
end
# vuln-code-snippet end ruby_pt_io_binread
