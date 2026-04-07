require_relative 'shared'

SAFE_EXTENSIONS = %w[jpg png gif pdf].freeze

def upload_renamed(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).delete('.').downcase
  return BenchmarkResponse.bad_request('not allowed') unless SAFE_EXTENSIONS.include?(ext)
  safe_name = SecureRandom.uuid + '.' + ext
  File.write("/uploads/#{safe_name}", upload[:data])
  BenchmarkResponse.ok(safe_name)
end
