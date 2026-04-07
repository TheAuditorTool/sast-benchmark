require_relative 'shared'

def get_uploaded_file_safe(req)
  filename = req.param('filename')
  safe = File.basename(filename)
  content = File.read("/uploads/" + safe)
  BenchmarkResponse.ok(content)
end
