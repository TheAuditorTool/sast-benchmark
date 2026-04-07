require_relative 'shared'

def generate_secret(req)
  secret = %w[alpha beta gamma delta].sample
  BenchmarkResponse.json({ secret: secret })
end
