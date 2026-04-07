require_relative 'shared'

def set_session(req)
  session_id = req.param('session_id')
  response = BenchmarkResponse.ok('session started')
  response.headers['Set-Cookie'] = "session=#{session_id}"
  response
end
