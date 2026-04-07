require_relative 'shared'

module Pundit
  def self.authorize(user, record, action); true; end
end

def update_post_safe(req)
  post_id = req.param('id')
  current_user = req.cookie('user_id')
  Pundit.authorize(current_user, { id: post_id }, :update?)
  BenchmarkResponse.ok("post #{post_id} updated")
end
