require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_no_validate
def upload_file(req)
  upload = req.file('file')
  original_name = upload[:filename]
  file_data = upload[:data]
  File.write("/uploads/#{original_name}", file_data) # vuln-code-snippet vuln-line ruby_fileupload_no_validate
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_fileupload_no_validate
