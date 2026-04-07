require_relative 'shared'

def render_dashboard(req)
  role = req.cookie('user_role')
  return BenchmarkResponse.error('Forbidden', 403) unless role == 'admin'
  BenchmarkResponse.ok('Admin dashboard')
end
