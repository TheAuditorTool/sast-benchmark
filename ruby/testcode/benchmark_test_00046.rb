require_relative 'shared'
require 'base64'

def resume_session(req)
  encoded = req.post('session')
  obj = Marshal.load(Base64.decode64(encoded))
  user_id = obj[:user_id].to_s
  BenchmarkResponse.ok("session for user #{user_id}")
end
