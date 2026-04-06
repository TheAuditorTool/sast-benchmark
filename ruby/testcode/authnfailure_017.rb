require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {}); [{ 'sub' => '1' }, { 'alg' => 'none' }]; end
end

# vuln-code-snippet start ruby_authn_jwt_none_alg
def authenticate_jwt_none(req)
  token = req.header('Authorization').sub('Bearer ', '')
  payload, header = JWT.decode(token, nil, false)
  BenchmarkResponse.json({ user_id: payload['sub'], alg: header['alg'] }) # vuln-code-snippet vuln-line ruby_authn_jwt_none_alg
end
# vuln-code-snippet end ruby_authn_jwt_none_alg
