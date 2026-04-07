require_relative 'shared'
require 'securerandom'

# This directory is mounted with the 'noexec' flag in /etc/fstab:
#   /dev/sdb1 /var/uploads ext4 defaults,noexec,nosuid 0 0
UPLOAD_DIR = '/var/uploads'.freeze

SAFE_EXTS_043 = %w[.jpg .jpeg .png .gif .pdf .txt].freeze

# vuln-code-snippet start ruby_fileupload_no_exec_dir
def upload_to_noexec_dir(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('type not allowed') unless SAFE_EXTS_043.include?(ext)

  safe_name = SecureRandom.uuid + ext
  dest      = File.join(UPLOAD_DIR, File.basename(safe_name))
  File.write(dest, upload[:data]) # vuln-code-snippet safe-line ruby_fileupload_no_exec_dir

  BenchmarkResponse.ok("stored")
end
# vuln-code-snippet end ruby_fileupload_no_exec_dir
