require_relative 'shared'
require 'zip'
require 'fileutils'

# vuln-code-snippet start ruby_fileupload_zip_slip
def upload_zip(req)
  upload = req.file('archive')
  return BenchmarkResponse.bad_request('no file') unless upload

  tmpfile = "/tmp/upload_#{Process.pid}.zip"
  File.write(tmpfile, upload[:data])
  dest_dir = '/var/uploads/extracted'
  FileUtils.mkdir_p(dest_dir)

  Zip::File.open(tmpfile) do |zip|
    zip.each do |entry|
      entry.extract(File.join(dest_dir, entry.name)) # vuln-code-snippet vuln-line ruby_fileupload_zip_slip
    end
  end

  File.delete(tmpfile)
  BenchmarkResponse.ok('archive extracted')
end
# vuln-code-snippet end ruby_fileupload_zip_slip
