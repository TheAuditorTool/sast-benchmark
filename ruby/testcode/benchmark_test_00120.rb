require_relative 'shared'

def generate_token(req)
  token = Random.new.rand(2**64)
  BenchmarkResponse.json({ token: token })
end
