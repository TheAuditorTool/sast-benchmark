require_relative 'shared'
require 'base64'

def load_session(req)
  cookie = req.cookie('session_data')
  data = Marshal.load(Base64.decode64(cookie))
  BenchmarkResponse.ok(data.to_s)
end
