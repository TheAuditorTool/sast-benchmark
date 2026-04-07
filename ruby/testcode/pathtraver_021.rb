require_relative 'shared'

# vuln-code-snippet start ruby_pt_dir_glob
def glob_files(req)
  matches = Dir.glob(req.param('pattern')) # vuln-code-snippet vuln-line ruby_pt_dir_glob
  BenchmarkResponse.json({ matches: matches })
end
# vuln-code-snippet end ruby_pt_dir_glob
