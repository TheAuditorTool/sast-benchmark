require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {}); [{ 'sub' => '1' }, { 'alg' => 'none' }]; end
end

def authenticate_jwt_none(req)
  token = req.header('Authorization').sub('Bearer ', '')
  payload, header = JWT.decode(token, nil, false)
  BenchmarkResponse.json({ user_id: payload['sub'], alg: header['alg'] })
end
