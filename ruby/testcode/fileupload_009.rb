require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_exec_ext
def upload_any_ext(req)
  upload = req.file('file')
  dest = "/uploads/#{upload[:filename]}" # vuln-code-snippet vuln-line ruby_fileupload_exec_ext
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_exec_ext
