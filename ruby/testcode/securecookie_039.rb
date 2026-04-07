require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_rack_full_flags
def build_rack_app
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('rack session set')
  response.headers['Set-Cookie'] = "rack.session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/" # vuln-code-snippet safe-line ruby_securecookie_rack_full_flags
  response
end
# vuln-code-snippet end ruby_securecookie_rack_full_flags
