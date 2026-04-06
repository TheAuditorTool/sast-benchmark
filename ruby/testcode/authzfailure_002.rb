require_relative 'shared'

# vuln-code-snippet start ruby_authz_scoped_query
def get_user_profile_safe(req)
  current_user_id = req.cookie('user_id')
  user_id = req.param('id')
  return BenchmarkResponse.error('forbidden', 403) unless current_user_id == user_id # vuln-code-snippet safe-line ruby_authz_scoped_query
  BenchmarkResponse.json({ id: user_id, email: "user#{user_id}@example.com" })
end
# vuln-code-snippet end ruby_authz_scoped_query
