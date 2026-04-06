require_relative 'shared'
require 'base64'

# vuln-code-snippet start ruby_deser_marshal_cookie
def load_session(req)
  cookie = req.cookie('session_data')
  data = Marshal.load(Base64.decode64(cookie)) # vuln-code-snippet vuln-line ruby_deser_marshal_cookie
  BenchmarkResponse.ok(data.to_s)
end
# vuln-code-snippet end ruby_deser_marshal_cookie
