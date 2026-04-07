require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_authn_predictable_reset
def generate_password_reset_token(req)
  username = req.post('username')
  token = Digest::MD5.hexdigest(username + Time.now.to_s) # vuln-code-snippet vuln-line ruby_authn_predictable_reset
  BenchmarkResponse.ok("Reset token: #{token}")
end
# vuln-code-snippet end ruby_authn_predictable_reset
