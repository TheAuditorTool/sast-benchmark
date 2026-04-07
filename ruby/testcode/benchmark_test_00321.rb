require_relative 'shared'

def fnmatch_read(req)
  content = File.read(req.param('path')) if File.fnmatch('*.txt', req.param('path'))
  BenchmarkResponse.ok(content.to_s)
end
