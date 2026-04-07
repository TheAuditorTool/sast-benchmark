require_relative 'shared'

def generate_session_id(req)
  session_id = Kernel.rand(2**128).to_s(16)
  BenchmarkResponse.ok(session_id)
end
