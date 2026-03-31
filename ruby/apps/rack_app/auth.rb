require_relative '../../testcode/shared'
require 'digest'
require 'securerandom'
begin; require 'bcrypt'; rescue LoadError; end

# rack_app - Authentication

# vuln-code-snippet start rk_sqli_concat
def login_unsafe(env)
  req = Rack::Request.new(env)
  username = req.params['username']
  password = req.params['password']
  db = get_db_connection
  rows = db.execute("SELECT id FROM users WHERE username = '#{username}' AND password = '#{password}'") # vuln-code-snippet vuln-line rk_sqli_concat
  return [401, { 'Content-Type' => 'text/plain' }, ['Unauthorized']] if rows.empty?
  [200, { 'Content-Type' => 'text/plain' }, ['OK']]
end
# vuln-code-snippet end rk_sqli_concat

# vuln-code-snippet start rk_sqli_prepared
def login_safe(env)
  req = Rack::Request.new(env)
  username = req.params['username']
  password = req.params['password']
  db = get_db_connection
  stmt = db.prepare('SELECT id FROM users WHERE username = ? AND password_hash = ?') # vuln-code-snippet safe-line rk_sqli_prepared
  rows = stmt.execute(username, password)
  return [401, { 'Content-Type' => 'text/plain' }, ['Unauthorized']] if rows.first.nil?
  [200, { 'Content-Type' => 'text/plain' }, ['OK']]
end
# vuln-code-snippet end rk_sqli_prepared

# vuln-code-snippet start rk_weakhash_md5
def hash_password_md5(env)
  req = Rack::Request.new(env)
  password = req.params['password']
  hashed = Digest::MD5.hexdigest(password) # vuln-code-snippet vuln-line rk_weakhash_md5
  [200, { 'Content-Type' => 'text/plain' }, [hashed]]
end
# vuln-code-snippet end rk_weakhash_md5

# vuln-code-snippet start rk_weakhash_bcrypt
def hash_password_bcrypt(env)
  req = Rack::Request.new(env)
  password = req.params['password']
  hashed = BCrypt::Password.create(password) # vuln-code-snippet safe-line rk_weakhash_bcrypt
  [200, { 'Content-Type' => 'text/plain' }, [hashed.to_s]]
end
# vuln-code-snippet end rk_weakhash_bcrypt

# vuln-code-snippet start rk_weakrand_session
def create_session_weak(env)
  req = Rack::Request.new(env)
  _user = req.params['user_id']
  token = rand(1_000_000_000).to_s # vuln-code-snippet vuln-line rk_weakrand_session
  [200, { 'Content-Type' => 'text/plain', 'Set-Cookie' => "session=#{token}" }, ['session created']]
end
# vuln-code-snippet end rk_weakrand_session

# vuln-code-snippet start rk_weakrand_secure
def create_session_secure(env)
  req = Rack::Request.new(env)
  _user = req.params['user_id']
  token = SecureRandom.hex(32) # vuln-code-snippet safe-line rk_weakrand_secure
  [200, { 'Content-Type' => 'text/plain', 'Set-Cookie' => "session=#{token}; HttpOnly; Secure; SameSite=Strict" }, ['session created']]
end
# vuln-code-snippet end rk_weakrand_secure

# vuln-code-snippet start rk_csrf_no_token
def update_email_no_csrf(env)
  req = Rack::Request.new(env)
  user_id = req.params['user_id']
  new_email = req.params['email']
  db = get_db_connection
  db.execute('UPDATE users SET email = ? WHERE id = ?', [new_email, user_id]) # vuln-code-snippet vuln-line rk_csrf_no_token
  [200, { 'Content-Type' => 'text/plain' }, ['email updated']]
end
# vuln-code-snippet end rk_csrf_no_token

# vuln-code-snippet start rk_csrf_verified
def update_email_csrf_verified(env)
  req = Rack::Request.new(env)
  submitted_token = req.params['csrf_token']
  session_token = req.cookies['csrf_token']
  return [403, { 'Content-Type' => 'text/plain' }, ['Forbidden']] unless submitted_token && submitted_token == session_token # vuln-code-snippet safe-line rk_csrf_verified
  db = get_db_connection
  db.execute('UPDATE users SET email = ? WHERE id = ?', [req.params['email'], req.params['user_id']])
  [200, { 'Content-Type' => 'text/plain' }, ['email updated']]
end
# vuln-code-snippet end rk_csrf_verified
