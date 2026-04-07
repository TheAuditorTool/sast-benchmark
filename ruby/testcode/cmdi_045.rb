require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_io_popen_array
def read_validated_file(req)
  filename = req.param('file').gsub(/[^a-z0-9._\-]/i, '')
  validated_path = File.join('/app/uploads', filename)
  content = IO.popen(['cat', '--', validated_path]) { |io| io.read }  # vuln-code-snippet safe-line ruby_cmdi_io_popen_array
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_cmdi_io_popen_array
