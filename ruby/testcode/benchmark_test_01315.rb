require_relative 'shared'

def upload_avatar(req)
  file = req.file('avatar')
  dest = "/var/app/uploads/#{file[:filename]}"
  File.binwrite(dest, file[:data])
  BenchmarkResponse.json({ path: dest })
end
