require_relative 'shared'

module Pundit
  def self.authorize(user, record, action); true; end
end

# vuln-code-snippet start ruby_authz_pundit_authorize
def update_post_safe(req)
  post_id = req.param('id')
  current_user = req.cookie('user_id')
  Pundit.authorize(current_user, { id: post_id }, :update?) # vuln-code-snippet safe-line ruby_authz_pundit_authorize
  BenchmarkResponse.ok("post #{post_id} updated")
end
# vuln-code-snippet end ruby_authz_pundit_authorize
