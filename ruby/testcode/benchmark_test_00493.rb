require_relative 'shared'

def upload_file(req)
  upload = req.file('file')
  original_name = upload[:filename]
  file_data = upload[:data]
  File.write("/uploads/#{original_name}", file_data)
  BenchmarkResponse.ok('uploaded')
end
