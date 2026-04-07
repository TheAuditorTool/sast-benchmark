require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_io_read_pipe
def read_file_contents(req)
  filename = req.param('file')
  content = IO.read("| cat #{filename}")  # vuln-code-snippet vuln-line ruby_cmdi_io_read_pipe
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_cmdi_io_read_pipe
