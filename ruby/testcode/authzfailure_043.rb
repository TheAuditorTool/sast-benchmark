require_relative 'shared'

class Forbidden < StandardError; end

class AdminUser
  attr_reader :id, :role

  def initialize(id, role)
    @id = id
    @role = role
  end

  def admin?
    @role == 'admin'
  end
end

def audit_logs
  [{ event: 'login', user: 'user_1' }, { event: 'export', user: 'user_2' }]
end

# vuln-code-snippet start ruby_authz_role_enum_check
def view_audit_logs(req)
  current_user = AdminUser.new(req.cookie('user_id'), req.cookie('role'))
  current_user.admin? || raise(Forbidden) # vuln-code-snippet safe-line ruby_authz_role_enum_check
  BenchmarkResponse.json(audit_logs)
end
# vuln-code-snippet end ruby_authz_role_enum_check
