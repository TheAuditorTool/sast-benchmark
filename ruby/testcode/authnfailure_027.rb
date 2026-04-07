require_relative 'shared'

# vuln-code-snippet start ruby_authn_admin_env
def require_admin(req)
  username = req.post('username')
  return BenchmarkResponse.ok("Admin access granted for #{username}") if ENV['ADMIN_BYPASS'] == 'true' # vuln-code-snippet vuln-line ruby_authn_admin_env
  BenchmarkResponse.error('Forbidden', 403)
end
# vuln-code-snippet end ruby_authn_admin_env
