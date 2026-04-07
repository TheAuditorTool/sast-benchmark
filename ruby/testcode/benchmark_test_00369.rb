require_relative 'shared'

def upload_document(req)
  filename = req.param('filename')
  content = req.post('content')
  File.write(File.join('/app/uploads', File.basename(filename)), content)
  BenchmarkResponse.json({ result: filename })
end
