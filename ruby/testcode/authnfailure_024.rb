require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_authn_remember_plain
def set_remember_me_cookie(req, cookies)
  user_id = req.post('user_id')
  generated_token = SecureRandom.hex(32)
  # Store plain token in DB here (omitted for brevity)
  cookies[:remember_token] = generated_token # vuln-code-snippet vuln-line ruby_authn_remember_plain
  BenchmarkResponse.ok('Remember-me cookie set')
end
# vuln-code-snippet end ruby_authn_remember_plain
