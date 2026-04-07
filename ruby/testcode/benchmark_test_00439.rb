require_relative 'shared'

ALLOWED = %w[report.pdf invoice.pdf summary.csv].freeze

def read_allowlisted(req)
  f = req.param('file')
  raise unless ALLOWED.include?(f)
  BenchmarkResponse.ok(File.read(File.join('/app/files', f)))
end
