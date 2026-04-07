require_relative 'shared'
require 'fileutils'
require 'zip'

SYM_DEST_DIR = '/var/uploads/extracted'.freeze

def extract_zip_with_symlinks(req)
  upload = req.file('archive')
  return BenchmarkResponse.bad_request('no file') unless upload

  tmpfile = "/tmp/upload_#{Process.pid}.zip"
  File.write(tmpfile, upload[:data])
  FileUtils.mkdir_p(SYM_DEST_DIR)

  Zip::File.open(tmpfile) do |zip|
    zip.each do |entry|
      src  = entry.extract(File.join('/tmp', 'zip_staging', entry.name)) { true }
      dest = File.join(SYM_DEST_DIR, entry.name)
      FileUtils.cp_r(src, dest)
    end
  end

  File.delete(tmpfile)
  BenchmarkResponse.ok('extracted')
end
