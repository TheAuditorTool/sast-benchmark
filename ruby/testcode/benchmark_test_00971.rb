require_relative 'shared'
require 'securerandom'

UPLOAD_BASE   = '/var/uploads'.freeze
MAX_TOTAL_MB  = 500

def upload_with_no_quota_check(req)
  upload  = req.file('file')
  user_id = req.post('user_id')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest = File.join(UPLOAD_BASE, user_id.to_s, SecureRandom.uuid + File.extname(upload[:filename]))
  File.write(dest, upload[:data])

  BenchmarkResponse.ok("stored #{upload[:data].bytesize} bytes")
end
