require_relative 'shared'

def get_tenant_data(req)
  tenant_id = req.param('tenant_id')
  BenchmarkResponse.json({ tenant: tenant_id, data: 'sensitive' })
end
