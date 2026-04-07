require_relative 'shared'

MIME_TO_EXT = { 'image/jpeg' => 'jpg', 'image/png' => 'png', 'application/pdf' => 'pdf' }.freeze

def upload_verified(req)
  upload = req.file('file')
  detected_mime = upload[:content_type]
  allowed_ext = MIME_TO_EXT[detected_mime]
  return BenchmarkResponse.bad_request('type not allowed') unless allowed_ext
  safe_name = SecureRandom.uuid + '.' + allowed_ext
  File.write("/uploads/#{safe_name}", upload[:data])
  BenchmarkResponse.ok(safe_name)
end
