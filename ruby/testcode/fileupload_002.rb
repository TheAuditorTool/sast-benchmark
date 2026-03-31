require_relative 'shared'

ALLOWED_EXTENSIONS = %w[jpg png gif pdf].freeze

# vuln-code-snippet start ruby_fileupload_ext_allowlist
def upload_file_safe(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).delete('.').downcase
  return BenchmarkResponse.bad_request('file type not allowed') unless ALLOWED_EXTENSIONS.include?(ext) # vuln-code-snippet safe-line ruby_fileupload_ext_allowlist
  File.write("/uploads/#{upload[:filename]}", upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_ext_allowlist
