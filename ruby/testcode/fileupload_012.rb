require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_fileupload_uuid_safe
def upload_uuid(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  safe_name = "#{SecureRandom.uuid}#{ext}" # vuln-code-snippet safe-line ruby_fileupload_uuid_safe
  dest = "/uploads/#{safe_name}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_uuid_safe
