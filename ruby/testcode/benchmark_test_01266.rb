require_relative 'shared'
require 'fileutils'

ALLOWED_EXTS = %w[.jpg .jpeg .png .gif .pdf].freeze
DEST_DIR     = '/var/uploads'.freeze

def upload_checked_move(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  filename = upload[:filename]
  ext      = File.extname(filename).downcase
  return BenchmarkResponse.bad_request('invalid type') unless ALLOWED_EXTS.include?(ext)

  tmppath  = "/tmp/upload_#{Process.pid}#{ext}"
  File.write(tmppath, upload[:data])

  dest_path = File.join(DEST_DIR, File.basename(filename))
  FileUtils.mv(tmppath, dest_path)

  BenchmarkResponse.ok("stored: #{dest_path}")
end
