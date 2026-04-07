require_relative 'shared'
require 'base64'

# vuln-code-snippet start ruby_deser_marshal_multipart
def multipart_deserialize_handler(req)
  raw = req.post('payload')
  obj = Marshal.load(Base64.decode64(raw))  # vuln-code-snippet vuln-line ruby_deser_marshal_multipart
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_marshal_multipart
