require_relative 'shared'

class Record
  def self.all
    [
      { id: 1, data: 'record_alpha', user_id: 'user_1' },
      { id: 2, data: 'record_beta',  user_id: 'user_2' },
      { id: 3, data: 'record_gamma', user_id: 'user_3' },
    ]
  end
end

# vuln-code-snippet start ruby_authz_bulk_export
def bulk_export(req)
  current_user = req.cookie('user_id')
  return BenchmarkResponse.error('unauthenticated', 401) unless current_user.present?
  records = Record.all # vuln-code-snippet vuln-line ruby_authz_bulk_export
  BenchmarkResponse.json(records)
end
# vuln-code-snippet end ruby_authz_bulk_export

class String
  def present?
    !empty?
  end
end
