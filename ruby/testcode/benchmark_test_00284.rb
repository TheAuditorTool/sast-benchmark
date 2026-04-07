require_relative 'shared'

module Current
  def self.tenant
    @tenant ||= Tenant.new('tenant_default')
  end

  def self.tenant=(t)
    @tenant = t
  end
end

class Tenant
  attr_reader :id

  def initialize(id)
    @id = id
  end
end

class TenantRecord
  def self.where(conditions)
    [{ id: 1, tenant_id: conditions[:tenant_id], payload: 'data' }]
  end
end

def list_tenant_records(req)
  records = TenantRecord.where(tenant_id: Current.tenant.id)
  BenchmarkResponse.json(records)
end
