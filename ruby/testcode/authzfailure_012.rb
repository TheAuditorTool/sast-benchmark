require_relative 'shared'

# vuln-code-snippet start ruby_authz_tenant_session
def get_tenant_safe(req)
  tenant_id = req.cookie('tenant_id') # vuln-code-snippet safe-line ruby_authz_tenant_session
  BenchmarkResponse.json({ tenant: tenant_id, data: 'sensitive' })
end
# vuln-code-snippet end ruby_authz_tenant_session
