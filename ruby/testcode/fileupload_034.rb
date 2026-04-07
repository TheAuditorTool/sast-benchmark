require_relative 'shared'
require 'securerandom'

ALLOWED_MIME_TYPES = %w[image/jpeg image/png application/pdf].freeze
SHRINE_UPLOAD_DIR  = '/var/uploads'.freeze

# Minimal Shrine-style uploader simulation.
class ShrineUploader
  ALLOWED_TYPES = ALLOWED_MIME_TYPES

  def self.determine_mime_type(data)
    header = data.byteslice(0, 8).b
    return 'image/jpeg' if header.start_with?("\xFF\xD8\xFF".b)
    return 'image/png'  if header.start_with?("\x89PNG".b)
    return 'application/pdf' if header.start_with?('%PDF')
    'application/octet-stream'
  end

  def self.validate_mime_type!(data)
    mime = determine_mime_type(data)
    raise ArgumentError, "MIME type #{mime} not allowed" unless ALLOWED_TYPES.include?(mime)
    mime
  end

  def self.upload(data, original_filename)
    mime = validate_mime_type!(data)
    ext  = { 'image/jpeg' => '.jpg', 'image/png' => '.png', 'application/pdf' => '.pdf' }.fetch(mime, '.bin')
    dest = File.join(SHRINE_UPLOAD_DIR, SecureRandom.uuid + ext)
    File.write(dest, data)
    dest
  end
end

# vuln-code-snippet start ruby_fileupload_shrineio_safe
def upload_shrine_safe(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest = ShrineUploader.upload(upload[:data], upload[:filename]) # vuln-code-snippet safe-line ruby_fileupload_shrineio_safe
  BenchmarkResponse.json({ stored: dest })
rescue ArgumentError => e
  BenchmarkResponse.bad_request(e.message)
end
# vuln-code-snippet end ruby_fileupload_shrineio_safe
