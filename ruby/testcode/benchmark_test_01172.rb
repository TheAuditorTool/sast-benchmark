require_relative 'shared'

def read_file_contents(req)
  filename = req.param('file')
  content = IO.read("| cat #{filename}")
  BenchmarkResponse.ok(content)
end
