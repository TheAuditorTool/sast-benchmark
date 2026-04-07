require_relative 'shared'

def upload_unlimited(req)
  upload = req.file('file')
  dest = "/uploads/#{Time.now.to_i}_#{upload[:filename]}"
  File.write(dest, upload[:data])
  BenchmarkResponse.ok('uploaded')
end
