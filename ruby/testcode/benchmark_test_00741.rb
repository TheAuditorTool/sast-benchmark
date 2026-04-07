require_relative 'shared'

def list_dir_safe(req)
  dir = req.param('dir')
  success = system("ls", "-la", dir)
  result = success ? "listing complete" : "command failed"
  BenchmarkResponse.ok(result)
end
