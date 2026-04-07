require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_fnmatch_read
def fnmatch_read(req)
  content = File.read(req.param('path')) if File.fnmatch('*.txt', req.param('path')) # vuln-code-snippet vuln-line ruby_pt_file_fnmatch_read
  BenchmarkResponse.ok(content.to_s)
end
# vuln-code-snippet end ruby_pt_file_fnmatch_read
