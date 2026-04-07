require_relative 'shared'

def load_checked_ext(req)
  input = req.param('plugin')
  name = File.basename(input)
  return BenchmarkResponse.bad_request('invalid') unless name.match?(/\A[a-z_]+\.rb\z/)
  full_path = File.expand_path(name, 'plugins')
  return BenchmarkResponse.bad_request('outside plugins') unless full_path.start_with?(File.expand_path('plugins'))
  load(full_path)
  BenchmarkResponse.ok("loaded #{name}")
end
