require_relative 'shared'
require 'securerandom'

CW_UPLOAD_DIR = '/var/uploads'.freeze

# CarrierWave-style uploader with extension allowlist.
class AvatarUploader
  def extension_allowlist
    %w[jpg png pdf]
  end

  def store!(upload)
    ext = File.extname(upload[:filename]).delete('.').downcase
    raise ArgumentError, "extension .#{ext} not allowed" unless extension_allowlist.include?(ext)

    dest = File.join(CW_UPLOAD_DIR, SecureRandom.uuid + '.' + ext)
    File.write(dest, upload[:data])
    dest
  end
end

# vuln-code-snippet start ruby_fileupload_carrierwave_allowlist
def upload_carrierwave_safe(req)
  upload  = req.file('avatar')
  return BenchmarkResponse.bad_request('no file') unless upload

  uploader = AvatarUploader.new
  dest     = uploader.store!(upload) # vuln-code-snippet safe-line ruby_fileupload_carrierwave_allowlist
  BenchmarkResponse.json({ url: "/uploads/#{File.basename(dest)}" })
rescue ArgumentError => e
  BenchmarkResponse.bad_request(e.message)
end
# vuln-code-snippet end ruby_fileupload_carrierwave_allowlist
