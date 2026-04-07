require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_expand_nocheck
def read_expanded_no_check(req)
  base = '/app/files'
  path = File.expand_path(req.param('path'), base) # vuln-code-snippet vuln-line ruby_pt_file_expand_nocheck
  BenchmarkResponse.ok(File.read(path))
end
# vuln-code-snippet end ruby_pt_file_expand_nocheck
