require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {}); [{ 'sub' => '1' }, {}]; end
end

def authenticate_jwt_safe(req)
  token = req.header('Authorization').sub('Bearer ', '')
  secret = ENV.fetch('JWT_SECRET')
  payload, _header = JWT.decode(token, secret, true, algorithm: 'HS256')
  BenchmarkResponse.json({ user_id: payload['sub'] })
end
