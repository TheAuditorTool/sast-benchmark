require_relative 'shared'
require 'securerandom'

STORE_DIR = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_no_antivirus
def upload_direct(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest_path = File.join(STORE_DIR, SecureRandom.uuid + File.extname(upload[:filename]))
  File.write(dest_path, upload[:data]) # vuln-code-snippet vuln-line ruby_fileupload_no_antivirus

  BenchmarkResponse.json({ stored: dest_path })
end
# vuln-code-snippet end ruby_fileupload_no_antivirus
