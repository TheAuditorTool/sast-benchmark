require_relative 'shared'

def generate_token_shuffle(req)
  charset = ('A'..'Z').to_a + ('a'..'z').to_a + ('0'..'9').to_a
  token = charset.shuffle.first(32).join
  BenchmarkResponse.ok(token)
end
