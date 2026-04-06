require_relative 'shared'
require 'securerandom'
require 'tempfile'

# vuln-code-snippet start ruby_fileupload_temp_rename
def upload_temp(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('invalid type') unless %w[.jpg .png .pdf].include?(ext)
  tmpfile = Tempfile.new(['upload', ext], '/var/tmp') # vuln-code-snippet safe-line ruby_fileupload_temp_rename
  tmpfile.write(upload[:data])
  tmpfile.close
  BenchmarkResponse.ok("stored: #{tmpfile.path}")
end
# vuln-code-snippet end ruby_fileupload_temp_rename
