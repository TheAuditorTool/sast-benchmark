require_relative 'shared'

def create_raw_params(req)
  params = FakeParams.new(req.post('record') || {})
  BenchmarkResponse.ok("created: #{params.to_h}")
end
