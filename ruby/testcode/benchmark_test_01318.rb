require_relative 'shared'

ALLOWED_MIME_TYPES = %w[application/pdf application/msword text/plain].freeze

def upload_document(req)
  file = req.file('document')
  mime = file[:content_type]
  return BenchmarkResponse.bad_request('type not allowed') unless ALLOWED_MIME_TYPES.include?(mime)
  safe_name = "#{SecureRandom.hex(12)}#{File.extname(file[:filename])}"
  dest = "/var/app/docs/#{safe_name}"
  File.binwrite(dest, file[:data])
  BenchmarkResponse.json({ stored: dest })
end
