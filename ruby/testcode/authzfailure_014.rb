require_relative 'shared'

# vuln-code-snippet start ruby_authz_default_scope
def list_posts(req)
  current_user = req.cookie('user_id')
  # default_scope { where(user_id: current_user) }
  BenchmarkResponse.json({ user: current_user, posts: [] }) # vuln-code-snippet safe-line ruby_authz_default_scope
end
# vuln-code-snippet end ruby_authz_default_scope
