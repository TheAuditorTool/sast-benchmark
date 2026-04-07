require_relative 'shared'

def generate_token(req)
  user_id = req.param('user_id')
  token = Digest::MD5.hexdigest("#{user_id}-reset")
  BenchmarkResponse.json({ token: token })
end
