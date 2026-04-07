require_relative 'shared'

# vuln-code-snippet start ruby_pt_dir_entries
def list_dir_entries(req)
  entries = Dir.entries(req.param('dir')) # vuln-code-snippet vuln-line ruby_pt_dir_entries
  BenchmarkResponse.json({ entries: entries })
end
# vuln-code-snippet end ruby_pt_dir_entries
