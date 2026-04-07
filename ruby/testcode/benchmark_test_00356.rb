require_relative 'shared'
require 'securerandom'

UPLOAD_DIR = '/var/uploads'.freeze

SAFE_EXTS_043 = %w[.jpg .jpeg .png .gif .pdf .txt].freeze

def upload_to_noexec_dir(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('type not allowed') unless SAFE_EXTS_043.include?(ext)

  safe_name = SecureRandom.uuid + ext
  dest      = File.join(UPLOAD_DIR, File.basename(safe_name))
  File.write(dest, upload[:data])

  BenchmarkResponse.ok("stored")
end
