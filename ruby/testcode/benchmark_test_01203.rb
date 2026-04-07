require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {})
    [{ 'sub' => '42', 'role' => 'user' }, { 'alg' => 'RS256' }]
  end
end

def authenticate_jwt_rs256(req)
  token = req.header('Authorization').sub('Bearer ', '')
  public_key = ENV.fetch('JWT_PUBLIC_KEY', 'rsa-public-key-placeholder')
  payload, _header = JWT.decode(token, public_key, true, algorithms: ['RS256'])
  BenchmarkResponse.json({ user_id: payload['sub'], role: payload['role'] })
end
