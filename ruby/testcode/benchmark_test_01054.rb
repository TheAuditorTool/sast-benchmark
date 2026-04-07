require_relative 'shared'

def read_regex_validated(req)
  f = req.param('f')
  raise unless f =~ /\A[a-z0-9_\-]+\.(pdf|csv|txt)\z/
  BenchmarkResponse.ok(File.read(File.join('/app/files', f)))
end
