require_relative 'shared'

def download_file(req)
  file_id = req.param('file_id')
  path = "/uploads/#{file_id}"
  BenchmarkResponse.ok("downloading: #{path}")
end
