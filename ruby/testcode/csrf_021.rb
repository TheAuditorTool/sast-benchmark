require_relative 'shared'

# vuln-code-snippet start ruby_csrf_cookie_only
def set_insecure_session(req)
  user_id = req.post('user_id')
  # SameSite=None without Secure flag — allows cross-site requests with cookie
  response_headers = { 'Set-Cookie' => "session=#{user_id}; SameSite=None; Path=/" }  # vuln-code-snippet vuln-line ruby_csrf_cookie_only
  BenchmarkResponse.json({ result: response_headers })
end
# vuln-code-snippet end ruby_csrf_cookie_only
