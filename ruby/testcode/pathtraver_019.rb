require_relative 'shared'
require 'pathname'

# vuln-code-snippet start ruby_pt_pathname_read
def read_pathname(req)
  content = Pathname.new(req.param('path')).read # vuln-code-snippet vuln-line ruby_pt_pathname_read
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_pathname_read
