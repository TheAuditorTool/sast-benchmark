require_relative 'shared'
require 'cgi'
require 'securerandom'

ESC_UPLOAD_DIR = '/var/uploads'.freeze

def upload_and_confirm_safe(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  filename     = upload[:filename]
  safe_name    = SecureRandom.uuid + File.extname(filename)
  dest         = File.join(ESC_UPLOAD_DIR, safe_name)
  File.write(dest, upload[:data])

  escaped_name = CGI.escapeHTML(filename)
  html         = "<html><body><p>File '#{escaped_name}' uploaded successfully.</p></body></html>"

  BenchmarkResponse.html(html)
end
