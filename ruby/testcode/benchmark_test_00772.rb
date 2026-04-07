require_relative 'shared'

def download_file(req)
  filename = req.param('filename')
  BenchmarkResponse.new(200, 'data', { 'Content-Disposition' => "attachment; filename=\"#{filename}\"" })
end
