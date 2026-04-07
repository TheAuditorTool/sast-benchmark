require_relative 'shared'
require 'securerandom'
require 'tempfile'

AV_UPLOAD_DIR = '/var/uploads'.freeze

def upload_with_av_scan(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  tmpfile = Tempfile.new(['av_scan', File.extname(upload[:filename])], '/tmp')
  tmpfile.binmode
  tmpfile.write(upload[:data])
  tmpfile.flush

  scan_passed = system('clamscan', '--no-summary', tmpfile.path)
  tmpfile.close

  unless scan_passed
    tmpfile.unlink
    return BenchmarkResponse.bad_request('file rejected by antivirus')
  end

  ext  = File.extname(upload[:filename]).downcase
  dest = File.join(AV_UPLOAD_DIR, SecureRandom.uuid + ext)
  File.write(dest, upload[:data])
  tmpfile.unlink

  BenchmarkResponse.ok("stored: #{dest}")
end
