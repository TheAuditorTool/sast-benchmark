require_relative 'shared'

SERVE_DIR = '/var/uploads'.freeze

def serve_upload(req)
  filename = req.param('filename')
  filepath = File.join(SERVE_DIR, File.basename(filename))

  return BenchmarkResponse.bad_request('not found') unless File.exist?(filepath)

  data    = File.binread(filepath)
  headers = {}
  headers['Content-Disposition'] = "inline; filename=\"#{filename}\""
  headers['Content-Type']        = 'application/octet-stream'

  BenchmarkResponse.new(200, data, headers)
end
