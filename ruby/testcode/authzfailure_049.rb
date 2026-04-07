require_relative 'shared'

class BulkRecord
  def self.joins_users_where(user_id:)
    [
      { id: 1, payload: 'alpha', user_id: user_id },
      { id: 2, payload: 'beta',  user_id: user_id },
    ]
  end
end

# vuln-code-snippet start ruby_authz_bulk_with_ownership
def bulk_download(req)
  current_user_id = req.cookie('user_id')
  records = BulkRecord.joins_users_where(user_id: current_user_id) # vuln-code-snippet safe-line ruby_authz_bulk_with_ownership
  BenchmarkResponse.json(records)
end
# vuln-code-snippet end ruby_authz_bulk_with_ownership
