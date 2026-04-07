require_relative 'shared'

ALLOWED_MIMES = %w[image/jpeg image/png image/gif application/pdf].freeze

def upload_mime_check(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('type not allowed') unless ALLOWED_MIMES.include?(upload[:content_type])
  original_name = upload[:filename]
  File.write("/uploads/#{original_name}", upload[:data])
  BenchmarkResponse.ok('uploaded')
end
