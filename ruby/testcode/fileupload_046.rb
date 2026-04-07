require_relative 'shared'
require 'cgi'
require 'securerandom'

ESC_UPLOAD_DIR = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_filename_escaped
def upload_and_confirm_safe(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  filename     = upload[:filename]
  safe_name    = SecureRandom.uuid + File.extname(filename)
  dest         = File.join(ESC_UPLOAD_DIR, safe_name)
  File.write(dest, upload[:data])

  escaped_name = CGI.escapeHTML(filename) # vuln-code-snippet safe-line ruby_fileupload_filename_escaped
  html         = "<html><body><p>File '#{escaped_name}' uploaded successfully.</p></body></html>"

  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_fileupload_filename_escaped
