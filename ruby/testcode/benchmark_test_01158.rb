require_relative 'shared'

def get_uploaded_file(req)
  filename = req.param('filename')
  content = File.read("/uploads/" + filename)
  BenchmarkResponse.ok(content)
end
