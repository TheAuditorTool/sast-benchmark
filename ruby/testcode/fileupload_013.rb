require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_null_byte
def upload_null_byte(req)
  upload = req.file('file')
  filename = upload[:filename]
  dest = "/uploads/#{filename}" # vuln-code-snippet vuln-line ruby_fileupload_null_byte
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_null_byte
