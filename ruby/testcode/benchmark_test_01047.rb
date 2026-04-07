require_relative 'shared'
require 'base64'

def cookie_marshal_deserialize_handler(req)
  cookie_data = req.cookie('session_data')
  obj = Marshal.load(Base64.decode64(cookie_data))
  BenchmarkResponse.json({ result: obj.to_s })
end
