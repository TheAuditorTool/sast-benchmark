require_relative 'shared'

# vuln-code-snippet start ruby_pt_glob_fixed_pattern
def glob_fixed_pattern(req)
  files = Dir.glob('/app/data/*.csv')
  safe_prefix = req.param('prefix').gsub(/[^a-z0-9]/, '')
  result = files.select { |f| File.basename(f).start_with?(safe_prefix) }
  BenchmarkResponse.json({ files: result }) # vuln-code-snippet safe-line ruby_pt_glob_fixed_pattern
end
# vuln-code-snippet end ruby_pt_glob_fixed_pattern
