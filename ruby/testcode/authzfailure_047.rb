require_relative 'shared'

class SoftDeletableRecord
  def self.where(conditions)
    results = [
      { id: 1, name: 'record_one',   deleted_at: nil },
      { id: 3, name: 'record_three', deleted_at: nil },
    ]
    conditions.each do |k, v|
      results = results.select { |r| r[k] == v }
    end
    results
  end

  def self.find_active(id)
    where(id: id.to_i, deleted_at: nil).first
  end
end

# vuln-code-snippet start ruby_authz_soft_delete_scope
def get_active_record(req)
  record_id = req.param('id')
  record = SoftDeletableRecord.find_active(record_id) # vuln-code-snippet safe-line ruby_authz_soft_delete_scope
  return BenchmarkResponse.error('not found', 404) unless record
  BenchmarkResponse.json(record)
end
# vuln-code-snippet end ruby_authz_soft_delete_scope
