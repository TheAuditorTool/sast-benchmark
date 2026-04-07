require_relative 'shared'
require 'securerandom'

XSS_UPLOAD_DIR = '/var/uploads'.freeze

def upload_and_confirm(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  filename = upload[:filename]
  dest     = File.join(XSS_UPLOAD_DIR, SecureRandom.uuid + File.extname(filename))
  File.write(dest, upload[:data])

  html = "<html><body><p>File '#{filename}' uploaded successfully.</p></body></html>"
  BenchmarkResponse.html(html)
end
