require_relative 'shared'
require 'securerandom'

EXT_FROM_MIME = {
  'image/jpeg'      => '.jpg',
  'image/png'       => '.png',
  'application/pdf' => '.pdf',
  'image/svg+xml'   => '.svg',
  'text/html'       => '.html'
}.freeze

UPLOAD_STORE = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_content_sniff
def upload_by_content_type(req)
  upload       = req.file('file')
  content_type = req.header('Content-Type')

  return BenchmarkResponse.bad_request('no file') unless upload

  ext       = EXT_FROM_MIME.fetch(content_type, '.bin')
  dest_path = File.join(UPLOAD_STORE, SecureRandom.uuid + ext)
  File.write(dest_path, upload[:data]) # vuln-code-snippet vuln-line ruby_fileupload_content_sniff

  BenchmarkResponse.json({ path: dest_path, type: content_type })
end
# vuln-code-snippet end ruby_fileupload_content_sniff
