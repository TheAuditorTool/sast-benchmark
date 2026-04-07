require_relative 'shared'

UPLOAD_PATH = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_php_disguise
def upload_with_ext_check(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  filename = upload[:filename]
  ext      = File.extname(filename).downcase

  return BenchmarkResponse.bad_request('only jpg/png allowed') unless %w[.jpg .png].include?(ext)

  dest = File.join(UPLOAD_PATH, filename)
  File.write(dest, upload[:data]) # vuln-code-snippet vuln-line ruby_fileupload_php_disguise

  BenchmarkResponse.ok("uploaded: #{filename}")
end
# vuln-code-snippet end ruby_fileupload_php_disguise
