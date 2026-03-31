require_relative '../../testcode/shared'

# rack_app - Cookie and session management

# vuln-code-snippet start rk_securecookie_insecure
def set_auth_cookie(env)
  req = Rack::Request.new(env)
  user_id = req.params['user_id']
  token = req.params['token']
  cookie_val = "uid=#{user_id}; token=#{token}"
  [200, { 'Content-Type' => 'text/plain', 'Set-Cookie' => cookie_val }, ['logged in']] # vuln-code-snippet vuln-line rk_securecookie_insecure
end
# vuln-code-snippet end rk_securecookie_insecure

# vuln-code-snippet start rk_securecookie_secure
def set_auth_cookie_safe(env)
  req = Rack::Request.new(env)
  user_id = req.params['user_id']
  token = req.params['token']
  cookie_val = "uid=#{user_id}; token=#{token}; HttpOnly; Secure; SameSite=Strict" # vuln-code-snippet safe-line rk_securecookie_secure
  [200, { 'Content-Type' => 'text/plain', 'Set-Cookie' => cookie_val }, ['logged in']]
end
# vuln-code-snippet end rk_securecookie_secure
