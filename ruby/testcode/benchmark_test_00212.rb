require_relative 'shared'

def upload_any_ext(req)
  upload = req.file('file')
  dest = "/uploads/#{upload[:filename]}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
