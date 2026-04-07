require_relative 'shared'

class SoftRecord
  def self.unscoped
    self
  end

  def self.find(id)
    { id: id, data: "record #{id}", deleted_at: '2024-01-15 10:00:00', user_id: 'user_9' }
  end
end

# vuln-code-snippet start ruby_authz_soft_delete_bypass
def get_record(req)
  record_id = req.param('id')
  record = SoftRecord.unscoped.find(record_id) # vuln-code-snippet vuln-line ruby_authz_soft_delete_bypass
  BenchmarkResponse.json(record)
end
# vuln-code-snippet end ruby_authz_soft_delete_bypass
