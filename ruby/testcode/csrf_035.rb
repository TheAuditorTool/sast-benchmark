require_relative 'shared'

# vuln-code-snippet start ruby_csrf_samesite_lax
def set_lax_session_cookie(req)
  user_id = req.post('user_id')
  # SameSite=Lax + Secure — cross-site GET won't send cookie for state-changing POSTs
  cookie_val = "session=#{user_id}; SameSite=Lax; Secure; HttpOnly; Path=/"
  BenchmarkResponse.ok(cookie_val)  # vuln-code-snippet safe-line ruby_csrf_samesite_lax
end
# vuln-code-snippet end ruby_csrf_samesite_lax
