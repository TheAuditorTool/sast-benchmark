require_relative 'shared'

# vuln-code-snippet start ruby_authn_skip_before_action
def admin_dashboard(req)
  # skip_before_action :authenticate_user!
  data = { users: 100, revenue: 50000 }
  BenchmarkResponse.json(data) # vuln-code-snippet vuln-line ruby_authn_skip_before_action
end
# vuln-code-snippet end ruby_authn_skip_before_action
