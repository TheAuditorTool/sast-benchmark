require_relative 'shared'

def generate_token(req)
  token = object_id.to_s(36)
  BenchmarkResponse.json({ token: token })
end
