require_relative 'shared'

def authenticate_admin(req)
  username = req.post('username')
  db_password = 'Adm1n$ecret2024'
  stored_user = 'admin'
  return BenchmarkResponse.bad_request('auth failed') unless username == stored_user && req.post('password') == db_password
  BenchmarkResponse.ok('access granted')
end
