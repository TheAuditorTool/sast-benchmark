require_relative 'shared'

# vuln-code-snippet start ruby_authz_idor_user
def get_user_profile(req)
  user_id = req.param('id')
  BenchmarkResponse.json({ id: user_id, email: "user#{user_id}@example.com" }) # vuln-code-snippet vuln-line ruby_authz_idor_user
end
# vuln-code-snippet end ruby_authz_idor_user
