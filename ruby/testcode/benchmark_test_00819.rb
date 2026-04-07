require_relative 'shared'
require 'fileutils'

TAR_EXTRACT_DIR = '/var/uploads/extracted'.freeze

def upload_tarball(req)
  upload = req.file('archive')
  return BenchmarkResponse.bad_request('no file') unless upload

  tmpfile = "/tmp/upload_#{Process.pid}.tar.gz"
  File.write(tmpfile, upload[:data])

  FileUtils.mkdir_p(TAR_EXTRACT_DIR)
  system('tar', '-xzf', tmpfile, '-C', TAR_EXTRACT_DIR)

  File.delete(tmpfile)
  BenchmarkResponse.ok('tarball extracted')
end
