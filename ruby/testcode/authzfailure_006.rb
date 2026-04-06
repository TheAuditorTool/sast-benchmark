require_relative 'shared'

# vuln-code-snippet start ruby_authz_require_admin
def admin_delete_safe(req)
  role = req.cookie('role')
  return BenchmarkResponse.error('forbidden', 403) unless role == 'admin' # vuln-code-snippet safe-line ruby_authz_require_admin
  user_id = req.param('id')
  BenchmarkResponse.ok("user #{user_id} deleted")
end
# vuln-code-snippet end ruby_authz_require_admin
