require_relative 'shared'

def get_tenant_safe(req)
  tenant_id = req.cookie('tenant_id')
  BenchmarkResponse.json({ tenant: tenant_id, data: 'sensitive' })
end
