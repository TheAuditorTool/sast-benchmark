require_relative 'shared'

def update_user_unsafe(req)
  params = FakeParams.new(req.post('user') || {})
  attrs = params.to_unsafe_h
  BenchmarkResponse.ok("updated: #{attrs}")
end
