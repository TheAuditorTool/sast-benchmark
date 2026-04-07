require_relative 'shared'
require 'securerandom'

def upload_uuid(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  safe_name = "#{SecureRandom.uuid}#{ext}"
  dest = "/uploads/#{safe_name}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
