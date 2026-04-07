require_relative 'shared'

def generate_token(req)
  token = "#{Time.now.nsec}#{rand}"
  BenchmarkResponse.json({ token: token })
end
