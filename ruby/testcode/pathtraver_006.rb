require_relative 'shared'

# vuln-code-snippet start ruby_pt_join_basename
def serve_asset(req)
  base_dir = "/assets/files"
  filename = req.param('filename')
  full_path = File.join(base_dir, File.basename(filename)) # vuln-code-snippet safe-line ruby_pt_join_basename
  content = File.read(full_path)
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_join_basename
