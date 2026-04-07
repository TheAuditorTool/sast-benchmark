require_relative 'shared'
require 'fileutils'

TAR_EXTRACT_DIR = '/var/uploads/extracted'.freeze

# vuln-code-snippet start ruby_fileupload_tar_slip
def upload_tarball(req)
  upload = req.file('archive')
  return BenchmarkResponse.bad_request('no file') unless upload

  tmpfile = "/tmp/upload_#{Process.pid}.tar.gz"
  File.write(tmpfile, upload[:data])

  FileUtils.mkdir_p(TAR_EXTRACT_DIR)
  system('tar', '-xzf', tmpfile, '-C', TAR_EXTRACT_DIR) # vuln-code-snippet vuln-line ruby_fileupload_tar_slip

  File.delete(tmpfile)
  BenchmarkResponse.ok('tarball extracted')
end
# vuln-code-snippet end ruby_fileupload_tar_slip
