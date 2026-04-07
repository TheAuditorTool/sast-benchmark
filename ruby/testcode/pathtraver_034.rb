require_relative 'shared'
require 'pathname'

# vuln-code-snippet start ruby_pt_cleanpath_prefix_check
def read_cleanpath_check(req)
  base = '/app/files'
  p = Pathname.new(base).join(req.param('rel')).cleanpath
  raise 'traversal' unless p.to_s.start_with?(base)
  p.read # vuln-code-snippet safe-line ruby_pt_cleanpath_prefix_check
  BenchmarkResponse.ok(p.read)
end
# vuln-code-snippet end ruby_pt_cleanpath_prefix_check
