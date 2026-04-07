require_relative 'shared'
require 'pathname'

# vuln-code-snippet start ruby_pt_pathname_ascend_check
def read_no_dotdot(req)
  joined = File.join('/app/files', req.param('rel'))
  raise if Pathname.new(joined).each_filename.any? { |f| f == '..' }
  BenchmarkResponse.ok(File.read(joined)) # vuln-code-snippet safe-line ruby_pt_pathname_ascend_check
end
# vuln-code-snippet end ruby_pt_pathname_ascend_check
