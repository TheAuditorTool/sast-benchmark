require_relative 'shared'

def read_safe(req)
  input = req.param('file')
  base = '/var/data'
  full = Pathname.new(File.join(base, input)).cleanpath.to_s
  return BenchmarkResponse.bad_request('outside base') unless full.start_with?(base)
  BenchmarkResponse.ok(File.read(full))
end
