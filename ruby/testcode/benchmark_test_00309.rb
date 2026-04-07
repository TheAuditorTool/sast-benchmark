require_relative 'shared'

def upload_double_ext(req)
  upload = req.file('file')
  filename = upload[:filename]
  dest = "/uploads/#{filename}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
