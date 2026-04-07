require_relative 'shared'
require 'securerandom'

PRIVATE_UPLOAD_DIR   = '/var/uploads'.freeze
ALLOWED_UPLOAD_EXTS  = %w[.jpg .jpeg .png .gif .pdf].freeze

# vuln-code-snippet start ruby_fileupload_random_store_path
def upload_with_random_name(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('type not allowed') unless ALLOWED_UPLOAD_EXTS.include?(ext)

  safe_path = File.join(PRIVATE_UPLOAD_DIR, SecureRandom.uuid + ext)
  File.write(safe_path, upload[:data]) # vuln-code-snippet safe-line ruby_fileupload_random_store_path

  BenchmarkResponse.ok("stored")
end
# vuln-code-snippet end ruby_fileupload_random_store_path
