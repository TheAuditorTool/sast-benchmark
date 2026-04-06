require_relative 'shared'

# vuln-code-snippet start ruby_authz_api_version_bypass
def api_v1_action(req)
  # /api/v1/admin/users -- no auth middleware on v1
  user_id = req.param('id')
  BenchmarkResponse.json({ id: user_id, role: 'admin' }) # vuln-code-snippet vuln-line ruby_authz_api_version_bypass
end
# vuln-code-snippet end ruby_authz_api_version_bypass
