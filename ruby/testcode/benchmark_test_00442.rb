require_relative 'shared'

def read_realpath_prefix(req)
  base = '/app/files'
  joined = File.join(base, req.param('rel'))
  rp = File.realpath(joined) rescue nil
  raise 'traversal' unless rp&.start_with?(base + '/')
  File.read(rp)
  BenchmarkResponse.ok(File.read(rp))
end
