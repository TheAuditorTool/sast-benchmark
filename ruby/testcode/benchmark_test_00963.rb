require_relative 'shared'
require 'securerandom'

def set_session(req)
  session_id = SecureRandom.uuid
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session_id=#{session_id}; Secure; HttpOnly; SameSite=Strict; Path=/"
  response
end
