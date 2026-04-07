require_relative 'shared'

ALLOWED = %w[report.pdf invoice.pdf summary.csv].freeze

# vuln-code-snippet start ruby_pt_allowlist_files
def read_allowlisted(req)
  f = req.param('file')
  raise unless ALLOWED.include?(f)
  BenchmarkResponse.ok(File.read(File.join('/app/files', f))) # vuln-code-snippet safe-line ruby_pt_allowlist_files
end
# vuln-code-snippet end ruby_pt_allowlist_files
