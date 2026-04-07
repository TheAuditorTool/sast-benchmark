require_relative 'shared'

def skip_authorization
  @_pundit_policy_authorized = true
end

def current_policy_name
  'ResourcePolicy'
end

def admin_dashboard(req)
  skip_authorization
  data = { users: 1042, revenue: 98_500 }
  BenchmarkResponse.json(data)
end
