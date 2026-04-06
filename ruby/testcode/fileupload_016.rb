require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_fileupload_private_dir
def upload_private(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  dest = "/var/private_uploads/#{SecureRandom.uuid}#{ext}" # vuln-code-snippet safe-line ruby_fileupload_private_dir
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded to private storage')
end
# vuln-code-snippet end ruby_fileupload_private_dir
