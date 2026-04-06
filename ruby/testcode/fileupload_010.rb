require_relative 'shared'
require 'securerandom'

SAFE_EXTENSIONS = %w[.jpg .jpeg .png .gif .pdf].freeze

# vuln-code-snippet start ruby_fileupload_ext_content_check
def upload_checked(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('invalid type') unless SAFE_EXTENSIONS.include?(ext) # vuln-code-snippet safe-line ruby_fileupload_ext_content_check
  dest = "/uploads/#{SecureRandom.uuid}#{ext}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_ext_content_check
