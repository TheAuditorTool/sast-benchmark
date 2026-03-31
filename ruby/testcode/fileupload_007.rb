require_relative 'shared'

ALLOWED_MIMES = %w[image/jpeg image/png image/gif application/pdf].freeze

# vuln-code-snippet start ruby_fileupload_mime_only
def upload_mime_check(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('type not allowed') unless ALLOWED_MIMES.include?(upload[:content_type])
  original_name = upload[:filename]
  File.write("/uploads/#{original_name}", upload[:data]) # vuln-code-snippet vuln-line ruby_fileupload_mime_only
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_mime_only
