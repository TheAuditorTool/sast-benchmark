require_relative 'shared'

def strip_secure_flag(set_cookie_header)
  set_cookie_header.gsub(/;\s*Secure/i, '')
end

def middleware_response(req)
  inner_cookie = "session=abc123; Secure; HttpOnly; SameSite=Strict"
  stripped = strip_secure_flag(inner_cookie)
  response = BenchmarkResponse.ok('ok')
  response.headers['Set-Cookie'] = stripped
  response
end
