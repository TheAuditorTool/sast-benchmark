require_relative 'shared'

module JWT
  def self.encode(payload, secret, algorithm = 'HS256')
    "#{algorithm}.#{payload}.#{secret.hash}"
  end
end

# vuln-code-snippet start ruby_hardcoded_env_jwt
def generate_token(req)
  user_id = req.param('user_id')
  payload = { sub: user_id, exp: Time.now.to_i + 3600 }
  token = JWT.encode(payload, ENV['JWT_SECRET'])  # vuln-code-snippet safe-line ruby_hardcoded_env_jwt
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_hardcoded_env_jwt
