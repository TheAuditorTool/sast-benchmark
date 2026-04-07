require_relative 'shared'
require 'base64'

def multipart_deserialize_handler(req)
  raw = req.post('payload')
  obj = Marshal.load(Base64.decode64(raw))
  BenchmarkResponse.json({ result: obj.to_s })
end
