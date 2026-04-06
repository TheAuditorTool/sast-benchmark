require_relative 'shared'

module ActionPolicy
  def self.authorize!(user, record, to:); true; end
end

# vuln-code-snippet start ruby_authz_action_policy
def update_with_policy(req)
  current_user = req.cookie('user_id')
  record_id = req.param('id')
  ActionPolicy.authorize!(current_user, { id: record_id }, to: :update?) # vuln-code-snippet safe-line ruby_authz_action_policy
  BenchmarkResponse.ok("record #{record_id} updated")
end
# vuln-code-snippet end ruby_authz_action_policy
