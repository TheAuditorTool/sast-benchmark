require_relative 'shared'

MAX_SIZE = 5 * 1024 * 1024

# vuln-code-snippet start ruby_fileupload_size_type
def upload_size_check(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('too large') if upload[:data].bytesize > MAX_SIZE
  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('invalid type') unless %w[.jpg .png .pdf].include?(ext) # vuln-code-snippet safe-line ruby_fileupload_size_type
  File.write("/uploads/#{upload[:filename]}", upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_size_type
