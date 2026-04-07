require_relative 'shared'

SERVE_BASE_DIR = '/var/uploads'.freeze

SAFE_SERVE_EXTS = %w[.jpg .jpeg .png .gif .pdf].freeze

def serve_file_as_attachment(req)
  filename = File.basename(req.param('filename'))
  filepath = File.join(SERVE_BASE_DIR, filename)

  return BenchmarkResponse.bad_request('not found') unless File.exist?(filepath)
  return BenchmarkResponse.bad_request('type not allowed') unless SAFE_SERVE_EXTS.include?(File.extname(filename).downcase)

  data    = File.binread(filepath)
  headers = {}
  headers['Content-Disposition'] = "attachment; filename=\"#{filename}\""
  headers['Content-Type']        = 'application/octet-stream'

  BenchmarkResponse.new(200, data, headers)
end
