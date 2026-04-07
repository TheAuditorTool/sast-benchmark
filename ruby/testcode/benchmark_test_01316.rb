require_relative 'shared'

ALLOWED_EXTENSIONS = %w[jpg jpeg png gif].freeze

def upload_avatar(req)
  file = req.file('avatar')
  ext = File.extname(file[:filename]).delete('.').downcase
  return BenchmarkResponse.bad_request('type not allowed') unless ALLOWED_EXTENSIONS.include?(ext)
  dest = "/var/app/uploads/#{SecureRandom.hex(16)}.#{ext}"
  File.binwrite(dest, file[:data])
  BenchmarkResponse.json({ path: dest })
end
