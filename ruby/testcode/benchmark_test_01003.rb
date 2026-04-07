require_relative 'shared'

def store_upload(req)
  upload = req.file('file')
  filename = upload[:filename]
  dest = File.join('/var/app/uploads', filename)
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('stored')
end
