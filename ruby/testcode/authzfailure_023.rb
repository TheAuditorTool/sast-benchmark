require_relative 'shared'

def skip_authorization
  # Pundit helper: marks action as authorized without any policy check
  @_pundit_policy_authorized = true
end

def current_policy_name
  'ResourcePolicy'
end

# vuln-code-snippet start ruby_authz_skip_pundit
def admin_dashboard(req)
  skip_authorization # vuln-code-snippet vuln-line ruby_authz_skip_pundit
  data = { users: 1042, revenue: 98_500 }
  BenchmarkResponse.json(data)
end
# vuln-code-snippet end ruby_authz_skip_pundit
