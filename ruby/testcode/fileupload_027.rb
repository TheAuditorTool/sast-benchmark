require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_public_dir
def upload_to_public(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest = File.join('public', 'uploads', upload[:filename])
  File.write(dest, upload[:data]) # vuln-code-snippet vuln-line ruby_fileupload_public_dir

  BenchmarkResponse.ok("accessible at /uploads/#{upload[:filename]}")
end
# vuln-code-snippet end ruby_fileupload_public_dir
