require_relative 'shared'

def generate_token(req)
  user_id = req.param('user_id')
  token = Digest::SHA256.hexdigest("#{user_id}-#{SecureRandom.hex(16)}")
  BenchmarkResponse.json({ token: token })
end
