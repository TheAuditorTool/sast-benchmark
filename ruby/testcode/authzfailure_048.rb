require_relative 'shared'

class OwnedReport
  def self.where(conditions)
    id = conditions[:id]
    uid = conditions[:user_id]
    match = "user_#{id.to_i % 5}" == uid
    match ? [{ id: id, title: "report_#{id}", user_id: uid }] : []
  end

  def self.first!(relation)
    raise 'not found' if relation.empty?
    relation.first
  end
end

# vuln-code-snippet start ruby_authz_report_owned
def get_owned_report(req)
  report_id = req.param('id')
  current_user_id = req.cookie('user_id')
  relation = OwnedReport.where(id: report_id, user_id: current_user_id) # vuln-code-snippet safe-line ruby_authz_report_owned
  report = OwnedReport.first!(relation)
  BenchmarkResponse.json(report)
end
# vuln-code-snippet end ruby_authz_report_owned
