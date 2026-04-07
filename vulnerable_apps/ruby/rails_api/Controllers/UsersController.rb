require_relative '../../../testcode/shared'
require 'uri'
require 'securerandom'

# Rails API - UsersController
# Covers: redirect, csrf, securecookie

# vuln-code-snippet start ra_redirect_open
def users_redirect(req)
  url = req.param('url')
  BenchmarkResponse.redirect(url) # vuln-code-snippet vuln-line ra_redirect_open
end
# vuln-code-snippet end ra_redirect_open

# vuln-code-snippet start ra_redirect_internal
def users_redirect_safe(req)
  url = req.param('url')
  allowed_hosts = %w[app.example.com api.example.com]
  begin
    host = URI.parse(url).host
    return BenchmarkResponse.bad_request('external redirect not allowed') unless allowed_hosts.include?(host) # vuln-code-snippet safe-line ra_redirect_internal
  rescue URI::InvalidURIError
    return BenchmarkResponse.bad_request('invalid url')
  end
  BenchmarkResponse.redirect(url)
end
# vuln-code-snippet end ra_redirect_internal

# vuln-code-snippet start ra_csrf_skip_action
def users_update_email(req)
  user_id = req.param('user_id').to_i
  new_email = req.post('email')
  db = get_db_connection
  db.execute('UPDATE users SET email = ? WHERE id = ?', [new_email, user_id]) # vuln-code-snippet vuln-line ra_csrf_skip_action
  BenchmarkResponse.json({ status: 'email updated' })
end
# vuln-code-snippet end ra_csrf_skip_action

# vuln-code-snippet start ra_csrf_protect_exception
def users_update_email_safe(req)
  csrf_token = req.header('X-CSRF-Token')
  session_token = req.cookie('csrf_token')
  return BenchmarkResponse.error('invalid csrf token', 403) unless csrf_token == session_token && !csrf_token.empty? # vuln-code-snippet safe-line ra_csrf_protect_exception
  user_id = req.param('user_id')
  new_email = req.post('email')
  FakeActiveRecord::Base.where(id: user_id.to_i, email: new_email)
  BenchmarkResponse.json({ status: 'email updated' })
end
# vuln-code-snippet end ra_csrf_protect_exception

# vuln-code-snippet start ra_securecookie_no_flags
def users_set_session(req)
  user_id = req.param('user_id')
  response = BenchmarkResponse.ok('session started')
  response.headers['Set-Cookie'] = "user_session=#{user_id}" # vuln-code-snippet vuln-line ra_securecookie_no_flags
  response
end
# vuln-code-snippet end ra_securecookie_no_flags

# vuln-code-snippet start ra_securecookie_flags
def users_set_session_safe(req)
  user_id = req.param('user_id')
  session_token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('session started')
  response.headers['Set-Cookie'] = "user_session=#{session_token}; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet safe-line ra_securecookie_flags
  response
end
# vuln-code-snippet end ra_securecookie_flags
