require_relative 'shared'

# vuln-code-snippet start ruby_pt_regex_filename_only
def read_regex_validated(req)
  f = req.param('f')
  raise unless f =~ /\A[a-z0-9_\-]+\.(pdf|csv|txt)\z/
  BenchmarkResponse.ok(File.read(File.join('/app/files', f))) # vuln-code-snippet safe-line ruby_pt_regex_filename_only
end
# vuln-code-snippet end ruby_pt_regex_filename_only
