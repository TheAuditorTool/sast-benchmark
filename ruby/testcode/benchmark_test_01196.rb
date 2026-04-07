require_relative 'shared'

ALLOWED_REPORTS = ['summary.pdf', 'details.pdf', 'overview.pdf'].freeze

def download_report(req)
  filename = req.param('filename')
  return BenchmarkResponse.bad_request("File not allowed") unless ALLOWED_REPORTS.include?(filename)
  content = File.read("/reports/#{filename}")
  BenchmarkResponse.ok(content)
end
