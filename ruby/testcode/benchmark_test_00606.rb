require_relative 'shared'
require 'securerandom'

def upload_private(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  dest = "/var/private_uploads/#{SecureRandom.uuid}#{ext}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded to private storage')
end
