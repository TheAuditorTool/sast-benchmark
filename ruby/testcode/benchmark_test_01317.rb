require_relative 'shared'

def upload_document(req)
  file = req.file('document')
  content_type = req.header('Content-Type')
  dest = "/var/app/docs/#{file[:filename]}"
  File.binwrite(dest, file[:data])
  BenchmarkResponse.json({ stored: dest, type: content_type })
end
