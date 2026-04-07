require_relative 'shared'
require 'base64'

def load_cookie_session(req)
  cookie_data = req.cookie('session')
  obj = Marshal.restore(Base64.decode64(cookie_data))
  BenchmarkResponse.ok(obj.to_s)
end
