require_relative 'shared'

def authenticate_admin(req)
  username = req.post('username')
  password = req.post('password')
  expected_user = ENV['ADMIN_USERNAME']
  expected_pass = ENV['ADMIN_PASSWORD']
  return BenchmarkResponse.bad_request('auth failed') unless username == expected_user && password == expected_pass
  BenchmarkResponse.ok('access granted')
end
