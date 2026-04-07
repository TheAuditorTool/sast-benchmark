require_relative 'shared'

def generate_token_time(req)
  srand(Time.now.to_i)
  token = rand(999_999_999).to_s
  BenchmarkResponse.ok(token)
end
