require_relative 'shared'

def upload_to_public(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  dest = File.join('public', 'uploads', upload[:filename])
  File.write(dest, upload[:data])

  BenchmarkResponse.ok("accessible at /uploads/#{upload[:filename]}")
end
