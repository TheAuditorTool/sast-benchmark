require_relative 'shared'
require 'securerandom'

EXEC_UPLOAD_DIR = '/var/uploads'.freeze

def upload_script(req)
  upload = req.file('script')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest_path = File.join(EXEC_UPLOAD_DIR, SecureRandom.uuid + File.extname(upload[:filename]))
  File.write(dest_path, upload[:data])
  File.chmod(0755, dest_path)

  BenchmarkResponse.ok("stored: #{dest_path}")
end
