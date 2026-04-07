require_relative 'shared'

# vuln-code-snippet start ruby_pt_realpath_prefix_check
def read_realpath_prefix(req)
  base = '/app/files'
  joined = File.join(base, req.param('rel'))
  rp = File.realpath(joined) rescue nil
  raise 'traversal' unless rp&.start_with?(base + '/')
  File.read(rp) # vuln-code-snippet safe-line ruby_pt_realpath_prefix_check
  BenchmarkResponse.ok(File.read(rp))
end
# vuln-code-snippet end ruby_pt_realpath_prefix_check
