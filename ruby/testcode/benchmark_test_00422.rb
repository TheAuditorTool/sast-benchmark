require_relative 'shared'
require 'securerandom'
require 'tempfile'

def upload_temp(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('invalid type') unless %w[.jpg .png .pdf].include?(ext)
  tmpfile = Tempfile.new(['upload', ext], '/var/tmp')
  tmpfile.write(upload[:data])
  tmpfile.close
  BenchmarkResponse.ok("stored: #{tmpfile.path}")
end
