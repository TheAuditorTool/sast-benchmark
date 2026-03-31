require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_original_name
def store_upload(req)
  upload = req.file('file')
  filename = upload[:filename]
  dest = File.join('/var/app/uploads', filename) # vuln-code-snippet vuln-line ruby_fileupload_original_name
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('stored')
end
# vuln-code-snippet end ruby_fileupload_original_name
