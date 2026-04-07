require_relative 'shared'
require 'securerandom'

SIZE_LIMITS = {
  'image/jpeg'      => 5  * 1024 * 1024,
  'image/png'       => 5  * 1024 * 1024,
  'image/gif'       => 2  * 1024 * 1024,
  'application/pdf' => 10 * 1024 * 1024
}.freeze

TYPED_UPLOAD_DIR = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_size_per_type
def upload_with_size_limit(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  mime = upload[:content_type].to_s.split(';').first.strip

  max_bytes = SIZE_LIMITS.fetch(mime, 0)
  unless upload[:data].bytesize <= max_bytes
    return BenchmarkResponse.bad_request("file too large for type #{mime}")
  end

  ext  = { 'image/jpeg' => '.jpg', 'image/png' => '.png', 'image/gif' => '.gif', 'application/pdf' => '.pdf' }.fetch(mime, '.bin')
  dest = File.join(TYPED_UPLOAD_DIR, SecureRandom.uuid + ext)
  File.write(dest, upload[:data]) # vuln-code-snippet safe-line ruby_fileupload_size_per_type

  BenchmarkResponse.ok("stored: #{dest}")
end
# vuln-code-snippet end ruby_fileupload_size_per_type
