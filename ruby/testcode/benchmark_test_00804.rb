require_relative 'shared'

def generate_token_sample(req)
  chars = ('a'..'z').to_a + ('0'..'9').to_a
  token = chars.sample(32).join
  BenchmarkResponse.ok(token)
end
