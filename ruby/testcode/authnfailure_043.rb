require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {})
    [{ 'sub' => '42', 'exp' => Time.now.to_i + 3600 }, {}]
  end
end

# vuln-code-snippet start ruby_authn_jwt_exp_verified
def authenticate_jwt_with_exp(req)
  token = req.header('Authorization').sub('Bearer ', '')
  secret = ENV.fetch('JWT_SECRET', 'changeme')
  payload, _header = JWT.decode(token, secret, true, algorithms: ['HS256'])
  return BenchmarkResponse.error('Token expired', 401) unless payload['exp'] > Time.now.to_i # vuln-code-snippet safe-line ruby_authn_jwt_exp_verified
  BenchmarkResponse.json({ user_id: payload['sub'] })
end
# vuln-code-snippet end ruby_authn_jwt_exp_verified
