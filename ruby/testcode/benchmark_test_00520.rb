require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {})
    [{ 'sub' => '1', 'role' => 'admin' }, { 'alg' => 'none' }]
  end
end

def authenticate_jwt_no_alg_restrict(req)
  token = req.header('Authorization').sub('Bearer ', '')
  payload, _header = JWT.decode(token, nil, false)
  BenchmarkResponse.json({ user_id: payload['sub'], role: payload['role'] })
end
