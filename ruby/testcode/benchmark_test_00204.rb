require_relative 'shared'

def upload_magic_check(req)
  upload = req.file('file')
  magic = upload[:data][0..3].unpack1('H*')
  allowed_magic = %w[89504e47 ffd8ffe0 ffd8ffe1 25504446]
  return BenchmarkResponse.bad_request('invalid file type') unless allowed_magic.include?(magic)
  File.write("/uploads/#{SecureRandom.uuid}#{File.extname(upload[:filename])}", upload[:data])
  BenchmarkResponse.ok('uploaded')
end
