require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_no_ext_check
def save_upload(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename])
  dest = "/uploads/file_#{Time.now.to_i}#{ext}" # vuln-code-snippet vuln-line ruby_fileupload_no_ext_check
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('saved')
end
# vuln-code-snippet end ruby_fileupload_no_ext_check
