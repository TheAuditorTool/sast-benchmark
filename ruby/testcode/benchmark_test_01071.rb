require_relative 'shared'

PERMITTED_EXT = %w[jpg png gif pdf].freeze
STORAGE_ROOT = '/var/private/uploads'.freeze

def upload_safe_storage(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename]).delete('.').downcase
  return BenchmarkResponse.bad_request('not allowed') unless PERMITTED_EXT.include?(ext)
  safe_name = SecureRandom.uuid + '.' + ext
  File.write(File.join(STORAGE_ROOT, safe_name), upload[:data])
  BenchmarkResponse.ok(safe_name)
end
