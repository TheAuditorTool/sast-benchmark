require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_stat
def stat_file(req)
  size = File.stat(req.param('file')).size # vuln-code-snippet vuln-line ruby_pt_file_stat
  BenchmarkResponse.json({ size: size })
end
# vuln-code-snippet end ruby_pt_file_stat
