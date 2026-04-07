require_relative 'shared'

def get_document(req)
  base = "/data/reports"
  path = req.param('path')
  resolved = File.expand_path(path, base)
  return BenchmarkResponse.bad_request("Invalid path") unless resolved.start_with?(base + "/")
  content = File.read(resolved)
  BenchmarkResponse.ok(content)
end
