require_relative 'shared'

module ActionPolicy
  def self.authorize!(user, record, to:); true; end
end

def update_with_policy(req)
  current_user = req.cookie('user_id')
  record_id = req.param('id')
  ActionPolicy.authorize!(current_user, { id: record_id }, to: :update?)
  BenchmarkResponse.ok("record #{record_id} updated")
end
