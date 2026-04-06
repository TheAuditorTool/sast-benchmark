require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {}); [{ 'sub' => '1', 'role' => 'admin' }, {}]; end
end

# vuln-code-snippet start ruby_authn_jwt_unverified
def authenticate_jwt_noverify(req)
  token = req.header('Authorization').sub('Bearer ', '')
  payload, _header = JWT.decode(token, nil, false) # vuln-code-snippet vuln-line ruby_authn_jwt_unverified
  BenchmarkResponse.json({ user_id: payload['sub'], role: payload['role'] })
end
# vuln-code-snippet end ruby_authn_jwt_unverified
