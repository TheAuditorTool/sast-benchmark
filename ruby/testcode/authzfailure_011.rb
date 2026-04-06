require_relative 'shared'

# vuln-code-snippet start ruby_authz_tenant_param
def get_tenant_data(req)
  tenant_id = req.param('tenant_id')
  BenchmarkResponse.json({ tenant: tenant_id, data: 'sensitive' }) # vuln-code-snippet vuln-line ruby_authz_tenant_param
end
# vuln-code-snippet end ruby_authz_tenant_param
