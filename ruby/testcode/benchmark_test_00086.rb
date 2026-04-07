require_relative 'shared'

def update_user_safe(req)
  params = FakeParams.new(req.post('user') || {})
  safe_attrs = params.permit(:name, :email)
  BenchmarkResponse.ok("updated: #{safe_attrs.to_h}")
end
