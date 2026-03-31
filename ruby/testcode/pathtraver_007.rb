require_relative 'shared'

# vuln-code-snippet start ruby_pt_io_read
def fetch_export(req)
  export_name = req.param('export')
  data = IO.read("/tmp/exports/#{export_name}") # vuln-code-snippet vuln-line ruby_pt_io_read
  BenchmarkResponse.ok(data)
end
# vuln-code-snippet end ruby_pt_io_read
