require_relative 'shared'

module JWT
  def self.decode(token, key, verify, opts = {})
    [{ 'sub' => '42', 'exp' => Time.now.to_i - 3600 }, {}]
  end
end

# vuln-code-snippet start ruby_authn_jwt_exp_ignored
def authenticate_jwt_no_exp_check(req)
  token = req.header('Authorization').sub('Bearer ', '')
  secret = ENV.fetch('JWT_SECRET', 'changeme')
  payload, _header = JWT.decode(token, secret, true, algorithms: ['HS256'])
  BenchmarkResponse.json({ user_id: payload['sub'] }) # vuln-code-snippet vuln-line ruby_authn_jwt_exp_ignored
end
# vuln-code-snippet end ruby_authn_jwt_exp_ignored
