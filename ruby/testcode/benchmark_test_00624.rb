require_relative 'shared'
require 'securerandom'

def set_remember_me_cookie(req, cookies)
  user_id = req.post('user_id')
  generated_token = SecureRandom.hex(32)
  cookies[:remember_token] = generated_token
  BenchmarkResponse.ok('Remember-me cookie set')
end
