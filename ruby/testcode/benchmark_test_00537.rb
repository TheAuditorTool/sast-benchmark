require_relative 'shared'

def save_upload(req)
  upload = req.file('file')
  ext = File.extname(upload[:filename])
  dest = "/uploads/file_#{Time.now.to_i}#{ext}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('saved')
end
