require_relative 'shared'
require 'securerandom'

ALLOWED_CONTENT_TYPES = %w[image/png image/jpeg application/pdf].freeze
AS_UPLOAD_DIR         = '/var/uploads'.freeze

# ActiveStorage-style attachment validator.
class AttachmentValidator
  def self.validate!(upload)
    ct = upload[:content_type].to_s.split(';').first.strip
    unless ALLOWED_CONTENT_TYPES.include?(ct)
      raise ArgumentError, "content_type '#{ct}' not permitted"
    end
    ct
  end
end

# vuln-code-snippet start ruby_fileupload_active_storage_safe
def upload_active_storage_safe(req)
  upload = req.file('attachment')
  return BenchmarkResponse.bad_request('no file') unless upload

  ct   = AttachmentValidator.validate!(upload) # vuln-code-snippet safe-line ruby_fileupload_active_storage_safe
  ext  = { 'image/png' => '.png', 'image/jpeg' => '.jpg', 'application/pdf' => '.pdf' }.fetch(ct, '.bin')
  dest = File.join(AS_UPLOAD_DIR, SecureRandom.uuid + ext)
  File.write(dest, upload[:data])

  BenchmarkResponse.ok("stored: #{dest}")
rescue ArgumentError => e
  BenchmarkResponse.bad_request(e.message)
end
# vuln-code-snippet end ruby_fileupload_active_storage_safe
