require_relative 'shared'

def download_public_file(req)
  path = req.param('path')
  content = File.read("/public/#{path}")
  BenchmarkResponse.ok(content)
end
