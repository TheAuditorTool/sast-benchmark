require_relative 'shared'
require 'zip'
require 'pathname'
require 'fileutils'

ZIP_DEST_DIR = '/var/uploads/extracted'.freeze

def upload_zip_safe(req)
  upload = req.file('archive')
  return BenchmarkResponse.bad_request('no file') unless upload

  tmpfile  = "/tmp/upload_#{Process.pid}.zip"
  File.write(tmpfile, upload[:data])
  FileUtils.mkdir_p(ZIP_DEST_DIR)

  Zip::File.open(tmpfile) do |zip|
    zip.each do |entry|
      resolved = Pathname.new(File.join(ZIP_DEST_DIR, entry.name)).cleanpath.to_s
      next unless resolved.start_with?(ZIP_DEST_DIR + File::SEPARATOR)

      FileUtils.mkdir_p(File.dirname(resolved))
      entry.extract(resolved)
    end
  end

  File.delete(tmpfile)
  BenchmarkResponse.ok('extracted safely')
end
