require_relative 'shared'

# vuln-code-snippet start ruby_pt_basename
def get_uploaded_file_safe(req)
  filename = req.param('filename')
  safe = File.basename(filename)
  content = File.read("/uploads/" + safe) # vuln-code-snippet safe-line ruby_pt_basename
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_basename
