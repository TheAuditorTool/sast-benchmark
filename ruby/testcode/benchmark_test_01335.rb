require_relative 'shared'

def post_session_create(req)
  username = req.post('username')
  token = SecureRandom.hex(32)
  resp = BenchmarkResponse.ok('logged in')
  resp.set_cookie('session', token, path: '/')
  resp
end
