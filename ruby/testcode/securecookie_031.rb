require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_no_path_scope
def set_admin_token(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('admin token set')
  response.headers['Set-Cookie'] = "admin_token=#{token}; Path=/; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_no_path_scope
  response
end
# vuln-code-snippet end ruby_securecookie_no_path_scope
