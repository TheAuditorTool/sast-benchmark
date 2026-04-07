require_relative 'shared'

class AdminPanel
  def self.purge_inactive_users
    ['user_8', 'user_14', 'user_22']
  end
end

# vuln-code-snippet start ruby_authz_admin_check_only
def purge_users(req)
  current_user = req.cookie('user_id')
  return BenchmarkResponse.error('unauthenticated', 401) unless current_user.present? # vuln-code-snippet vuln-line ruby_authz_admin_check_only
  purged = AdminPanel.purge_inactive_users
  BenchmarkResponse.json({ purged: purged })
end
# vuln-code-snippet end ruby_authz_admin_check_only

class String
  def present?
    !empty?
  end
end
