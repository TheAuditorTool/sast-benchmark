require_relative 'shared'

def set_session(req)
  user_id = req.param('user_id')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session_id=user_#{user_id}_#{Time.now.to_i}; Secure; HttpOnly; SameSite=Strict"
  response
end
