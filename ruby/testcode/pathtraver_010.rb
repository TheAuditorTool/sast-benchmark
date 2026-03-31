require_relative 'shared'

# vuln-code-snippet start ruby_pt_allowlist
ALLOWED_REPORTS = ['summary.pdf', 'details.pdf', 'overview.pdf'].freeze

def download_report(req)
  filename = req.param('filename')
  return BenchmarkResponse.bad_request("File not allowed") unless ALLOWED_REPORTS.include?(filename) # vuln-code-snippet safe-line ruby_pt_allowlist
  content = File.read("/reports/#{filename}")
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_allowlist
