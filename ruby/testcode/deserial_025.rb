require_relative 'shared'
require 'base64'

# vuln-code-snippet start ruby_deser_cookie_marshal
def cookie_marshal_deserialize_handler(req)
  cookie_data = req.cookie('session_data')
  obj = Marshal.load(Base64.decode64(cookie_data))  # vuln-code-snippet vuln-line ruby_deser_cookie_marshal
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_cookie_marshal
