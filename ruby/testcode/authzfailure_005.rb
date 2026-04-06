require_relative 'shared'

# vuln-code-snippet start ruby_authz_admin_no_check
def admin_delete_user(req)
  user_id = req.param('id')
  # No role check
  BenchmarkResponse.ok("user #{user_id} deleted") # vuln-code-snippet vuln-line ruby_authz_admin_no_check
end
# vuln-code-snippet end ruby_authz_admin_no_check
