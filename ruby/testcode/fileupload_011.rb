require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_double_ext
def upload_double_ext(req)
  upload = req.file('file')
  filename = upload[:filename]
  dest = "/uploads/#{filename}" # vuln-code-snippet vuln-line ruby_fileupload_double_ext
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_double_ext
