require_relative 'shared'

module JWT
  def self.encode(payload, secret, algorithm = 'HS256')
    "#{algorithm}.#{payload}.#{secret.hash}"
  end
end

def generate_token(req)
  user_id = req.param('user_id')
  payload = { sub: user_id, exp: Time.now.to_i + 3600 }
  token = JWT.encode(payload, ENV['JWT_SECRET'])
  BenchmarkResponse.ok(token)
end
