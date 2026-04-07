require_relative 'shared'

# vuln-code-snippet start ruby_pt_expand_path_start_with
def read_expand_path_start_with(req)
  base = '/app/files'
  user_rel = req.param('path')
  p = File.expand_path(user_rel, base)
  raise unless p.start_with?(base + '/')
  BenchmarkResponse.ok(File.read(p)) # vuln-code-snippet safe-line ruby_pt_expand_path_start_with
end
# vuln-code-snippet end ruby_pt_expand_path_start_with
