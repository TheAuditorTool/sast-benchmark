require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_read
def get_uploaded_file(req)
  filename = req.param('filename')
  content = File.read("/uploads/" + filename) # vuln-code-snippet vuln-line ruby_pt_file_read
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_file_read
