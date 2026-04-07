require_relative 'shared'

def admin_dashboard(req)
  data = { users: 100, revenue: 50000 }
  BenchmarkResponse.json(data)
end
