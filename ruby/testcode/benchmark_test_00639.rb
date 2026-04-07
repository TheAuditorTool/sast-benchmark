require_relative 'shared'

def upload_null_byte(req)
  upload = req.file('file')
  filename = upload[:filename]
  dest = "/uploads/#{filename}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
