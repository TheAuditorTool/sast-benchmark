require_relative 'shared'
require 'securerandom'

STORE_DIR = '/var/uploads'.freeze

def upload_direct(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest_path = File.join(STORE_DIR, SecureRandom.uuid + File.extname(upload[:filename]))
  File.write(dest_path, upload[:data])

  BenchmarkResponse.json({ stored: dest_path })
end
