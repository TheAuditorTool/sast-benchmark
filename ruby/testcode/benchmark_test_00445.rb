require_relative 'shared'

def generate_token(req)
  token = Time.now.to_i.to_s(36)
  BenchmarkResponse.json({ token: token })
end
