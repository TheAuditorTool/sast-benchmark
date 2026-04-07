require_relative 'shared'

def set_insecure_session(req)
  user_id = req.post('user_id')
  response_headers = { 'Set-Cookie' => "session=#{user_id}; SameSite=None; Path=/" }
  BenchmarkResponse.json({ result: response_headers })
end
