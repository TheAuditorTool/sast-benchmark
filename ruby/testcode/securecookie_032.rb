require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_host_prefix
def set_session(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "__Host-session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/" # vuln-code-snippet safe-line ruby_securecookie_host_prefix
  response
end
# vuln-code-snippet end ruby_securecookie_host_prefix
