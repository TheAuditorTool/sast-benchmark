require_relative 'shared'
require 'securerandom'

SAFE_EXTENSIONS = %w[.jpg .jpeg .png .gif .pdf].freeze

def upload_checked(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).downcase
  return BenchmarkResponse.bad_request('invalid type') unless SAFE_EXTENSIONS.include?(ext)
  dest = "/uploads/#{SecureRandom.uuid}#{ext}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
