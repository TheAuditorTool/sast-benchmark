require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_host_prefix_missing
def set_session(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/" # vuln-code-snippet vuln-line ruby_securecookie_host_prefix_missing
  response
end
# vuln-code-snippet end ruby_securecookie_host_prefix_missing
