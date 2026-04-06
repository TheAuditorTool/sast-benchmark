require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_no_size_limit
def upload_unlimited(req)
  upload = req.file('file')
  dest = "/uploads/#{Time.now.to_i}_#{upload[:filename]}" # vuln-code-snippet vuln-line ruby_fileupload_no_size_limit
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_no_size_limit
