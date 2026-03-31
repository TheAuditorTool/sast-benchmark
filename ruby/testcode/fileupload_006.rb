require_relative 'shared'

MIME_TO_EXT = { 'image/jpeg' => 'jpg', 'image/png' => 'png', 'application/pdf' => 'pdf' }.freeze

# vuln-code-snippet start ruby_fileupload_content_check
def upload_verified(req)
  upload = req.file('file')
  detected_mime = upload[:content_type]
  allowed_ext = MIME_TO_EXT[detected_mime]
  return BenchmarkResponse.bad_request('type not allowed') unless allowed_ext # vuln-code-snippet safe-line ruby_fileupload_content_check
  safe_name = SecureRandom.uuid + '.' + allowed_ext
  File.write("/uploads/#{safe_name}", upload[:data])
  BenchmarkResponse.ok(safe_name)
end
# vuln-code-snippet end ruby_fileupload_content_check
