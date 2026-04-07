require_relative 'shared'

def list_directory(req)
  path = req.param('path')
  result = `ls
  BenchmarkResponse.ok(result)
end
