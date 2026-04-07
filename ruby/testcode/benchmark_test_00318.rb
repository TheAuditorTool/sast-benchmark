require_relative 'shared'

ALLOWED_EXTENSIONS = %w[jpg png gif pdf].freeze

def upload_file_safe(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).delete('.').downcase
  return BenchmarkResponse.bad_request('file type not allowed') unless ALLOWED_EXTENSIONS.include?(ext)
  File.write("/uploads/#{upload[:filename]}", upload[:data])
  BenchmarkResponse.ok('uploaded')
end
