require_relative 'shared'

def serve_asset(req)
  base_dir = "/assets/files"
  filename = req.param('filename')
  full_path = File.join(base_dir, File.basename(filename))
  content = File.read(full_path)
  BenchmarkResponse.ok(content)
end
