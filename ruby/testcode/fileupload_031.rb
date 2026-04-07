require_relative 'shared'
require 'securerandom'

EXEC_UPLOAD_DIR = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_exec_permission
def upload_script(req)
  upload = req.file('script')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest_path = File.join(EXEC_UPLOAD_DIR, SecureRandom.uuid + File.extname(upload[:filename]))
  File.write(dest_path, upload[:data])
  File.chmod(0755, dest_path) # vuln-code-snippet vuln-line ruby_fileupload_exec_permission

  BenchmarkResponse.ok("stored: #{dest_path}")
end
# vuln-code-snippet end ruby_fileupload_exec_permission
