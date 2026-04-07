require_relative 'shared'

SAFE_DIR = '/app/files'.freeze

# vuln-code-snippet start ruby_pt_basename_join_safe
def read_basename_join(req)
  path = File.join(SAFE_DIR, File.basename(req.param('file')))
  File.read(path) # vuln-code-snippet safe-line ruby_pt_basename_join_safe
  BenchmarkResponse.ok(File.read(path))
end
# vuln-code-snippet end ruby_pt_basename_join_safe
