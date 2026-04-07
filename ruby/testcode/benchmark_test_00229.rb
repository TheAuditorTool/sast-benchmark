require_relative 'shared'

def update_profile_safe(req)
  token = req.post('authenticity_token')
  session_token = req.cookie('csrf_token')
  return BenchmarkResponse.error('CSRF rejected', 422) unless token == session_token
  BenchmarkResponse.ok('profile updated')
end
