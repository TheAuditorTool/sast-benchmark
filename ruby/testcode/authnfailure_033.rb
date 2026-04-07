require_relative 'shared'

# vuln-code-snippet start ruby_authn_cookie_role_trust
def render_dashboard(req)
  role = req.cookie('user_role') # vuln-code-snippet vuln-line ruby_authn_cookie_role_trust
  return BenchmarkResponse.error('Forbidden', 403) unless role == 'admin'
  BenchmarkResponse.ok('Admin dashboard')
end
# vuln-code-snippet end ruby_authn_cookie_role_trust
