require_relative 'shared'
require 'openssl'

module JWT
  def self.decode(token, key, verify, opts = {}); [{ 'sub' => '1' }, {}]; end
end

# vuln-code-snippet start ruby_authn_jwt_rs256
def authenticate_jwt_rs256(req)
  token = req.header('Authorization').sub('Bearer ', '')
  public_key = OpenSSL::PKey::RSA.new(ENV.fetch('JWT_PUBLIC_KEY'))
  payload, _header = JWT.decode(token, public_key, true, algorithms: ['RS256']) # vuln-code-snippet safe-line ruby_authn_jwt_rs256
  BenchmarkResponse.json({ user_id: payload['sub'] })
end
# vuln-code-snippet end ruby_authn_jwt_rs256
